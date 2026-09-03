// SPDX-License-Identifier: GPL-3.0-or-later

//! Desktop notifications.
//!
//! Spoken directly over `org.freedesktop.Notifications` rather than through a
//! helper crate, so the app carries one D-Bus stack and one async runtime.
//!
//! Notifications are a convenience, never the mechanism: a device stays
//! blocked whether or not the notification is seen, and every action they
//! offer is also reachable from the main window.

use std::collections::HashMap;

use futures::StreamExt;
use zbus::zvariant::Value;

use crate::constants::{APP_ID, NOTIFY_INFO_TIMEOUT_MS, NOTIFY_PROMPT_TIMEOUT_MS};
use crate::debug_log;
use crate::usbguard::Device;

/// Action key sent back when the user allows a device from the notification.
pub const ACTION_ALLOW: &str = "allow";
/// Action key sent back when the user blocks a device from the notification.
pub const ACTION_BLOCK: &str = "block";
/// Action key sent back when the user asks to see the details first.
pub const ACTION_DETAILS: &str = "details";
/// Action key sent back from a refusal notice, asking to manage the device.
pub const ACTION_MANAGE: &str = "manage";

/// A connection to the notification daemon.
#[derive(Clone)]
pub struct Notifier {
    proxy: crate::usbguard::proxy::NotificationsProxy<'static>,
}

impl std::fmt::Debug for Notifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Notifier").finish_non_exhaustive()
    }
}

impl Notifier {
    /// Connect to the session bus notification service.
    ///
    /// Returns `None` when no notification daemon is available; the caller
    /// carries on without notifications rather than failing.
    pub async fn connect() -> Option<Self> {
        let connection = match zbus::Connection::session().await {
            Ok(connection) => connection,
            Err(e) => {
                debug_log!(crate::debug::NOTIFY, "no session bus: {e}");
                return None;
            }
        };
        match crate::usbguard::proxy::NotificationsProxy::new(&connection).await {
            Ok(proxy) => {
                debug_log!(crate::debug::NOTIFY, "notification proxy ready");
                Some(Self { proxy })
            }
            Err(e) => {
                debug_log!(crate::debug::NOTIFY, "no notification service: {e}");
                None
            }
        }
    }

    /// Ask the user about a newly inserted device.
    ///
    /// The notification never expires on its own: a pending security decision
    /// that quietly disappears is worse than no notification at all.
    pub async fn prompt(&self, device: &Device, replaces: u32) -> Option<u32> {
        let summary = crate::fl!("notify-new-device");
        let body = prompt_body(device);

        let urgency = Value::from(2u8); // critical
        let category = Value::from("device.added");
        let desktop_entry = Value::from(APP_ID);
        let mut hints: HashMap<&str, &Value<'_>> = HashMap::new();
        hints.insert("urgency", &urgency);
        hints.insert("category", &category);
        hints.insert("desktop-entry", &desktop_entry);

        let allow = crate::fl!("allow");
        let block = crate::fl!("block");
        let details = crate::fl!("details");
        let actions = [
            ACTION_ALLOW,
            allow.as_str(),
            ACTION_BLOCK,
            block.as_str(),
            ACTION_DETAILS,
            details.as_str(),
        ];

        self.send(
            replaces,
            "drive-removable-media-usb-symbolic",
            &summary,
            &body,
            &actions,
            hints,
            NOTIFY_PROMPT_TIMEOUT_MS,
        )
        .await
    }

    /// Tell the user a device was refused without them being asked.
    ///
    /// This is the case that is otherwise invisible: a device with a standing
    /// block or reject rule never raises a prompt, so from the user's side the
    /// drive simply does not appear and nothing says why. The notification
    /// carries a single action that opens the window at that device, because
    /// the answer to "why did that not work" is a thing they will want to
    /// change, and hunting for the app first is friction at exactly the wrong
    /// moment.
    ///
    /// Not urgent, and it expires on its own: unlike a pending decision, there
    /// is nothing outstanding here — the device is already refused.
    pub async fn auto_blocked(&self, device: &Device) -> Option<u32> {
        let summary = crate::fl!("notify-auto-blocked");
        let body = crate::fl!("notify-auto-blocked-body", name = device.display_name());

        let category = Value::from("device.error");
        let desktop_entry = Value::from(APP_ID);
        let mut hints: HashMap<&str, &Value<'_>> = HashMap::new();
        hints.insert("category", &category);
        hints.insert("desktop-entry", &desktop_entry);

        let manage = crate::fl!("notify-manage");
        let actions = [ACTION_MANAGE, manage.as_str()];

        self.send(
            0,
            "changes-prevent-symbolic",
            &summary,
            &body,
            &actions,
            hints,
            NOTIFY_INFO_TIMEOUT_MS,
        )
        .await
    }

    /// Post an informational notification with no actions.
    pub async fn info(&self, summary: &str, body: &str, icon: &str) -> Option<u32> {
        let category = Value::from("device");
        let desktop_entry = Value::from(APP_ID);
        let mut hints: HashMap<&str, &Value<'_>> = HashMap::new();
        hints.insert("category", &category);
        hints.insert("desktop-entry", &desktop_entry);

        self.send(0, icon, summary, body, &[], hints, NOTIFY_INFO_TIMEOUT_MS)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn send(
        &self,
        replaces: u32,
        icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value<'_>>,
        timeout: i32,
    ) -> Option<u32> {
        match self
            .proxy
            .notify(
                &crate::fl!("app-title"),
                replaces,
                icon,
                summary,
                body,
                actions,
                hints,
                timeout,
            )
            .await
        {
            Ok(id) => {
                debug_log!(crate::debug::NOTIFY, "posted notification {id}: {summary}");
                Some(id)
            }
            Err(e) => {
                debug_log!(crate::debug::NOTIFY, "could not post notification: {e}");
                None
            }
        }
    }

    /// Withdraw a notification.
    pub async fn close(&self, id: u32) {
        if id == 0 {
            return;
        }
        if let Err(e) = self.proxy.close_notification(id).await {
            debug_log!(
                crate::debug::NOTIFY,
                "could not close notification {id}: {e}"
            );
        }
    }

    /// Stream of `(notification_id, action_key)` for activated actions.
    ///
    /// Boxed so callers can hold it across awaits: the underlying zbus signal
    /// stream is not [`Unpin`].
    pub async fn actions(&self) -> Option<futures::stream::BoxStream<'static, (u32, String)>> {
        let stream = self.proxy.receive_action_invoked().await.ok()?;
        Some(
            stream
                .filter_map(|signal| async move {
                    let args = signal.args().ok()?;
                    Some((args.id, args.action_key.to_string()))
                })
                .boxed(),
        )
    }
}

/// Body text for a device prompt: enough to make a decision without opening
/// the app, and no more.
fn prompt_body(device: &Device) -> String {
    let mut lines = vec![device.display_name()];

    let id = device.usb_id();
    if !id.is_empty() {
        lines.push(id);
    }
    let classes = device.interface_classes();
    if !classes.is_empty() {
        lines.push(classes.join(", "));
    }
    if device.is_input_capable() {
        // Worth its own line: this is the property that makes an unknown
        // device dangerous rather than merely unwanted.
        lines.push(crate::fl!("warning-input-capable"));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(rule: &str) -> Device {
        Device::from_rule(1, rule).unwrap()
    }

    #[test]
    fn prompt_body_leads_with_the_device_name() {
        let body = prompt_body(&device(
            r#"block id 0781:5567 name "Cruzer Blade" with-interface 08:06:50"#,
        ));
        let mut lines = body.lines();
        assert_eq!(lines.next(), Some("Cruzer Blade"));
        assert_eq!(lines.next(), Some("0781:5567"));
        assert_eq!(lines.next(), Some("Mass storage"));
    }

    #[test]
    fn prompt_body_calls_out_input_capable_devices() {
        let body = prompt_body(&device(
            r#"block id 046d:c31c name "Keyboard" with-interface 03:01:01"#,
        ));
        assert!(body.contains("Human interface device"));
        // The keystroke-injection warning must be present, not merely implied
        // by the class name.
        assert!(body.lines().count() > 3, "missing warning line in: {body}");
    }

    #[test]
    fn every_notification_action_key_is_distinct() {
        // Two actions sharing a key would silently make one of them do the
        // other's job — and one of these authorises a device.
        let keys = [ACTION_ALLOW, ACTION_BLOCK, ACTION_DETAILS, ACTION_MANAGE];
        let unique: std::collections::HashSet<_> = keys.iter().collect();
        assert_eq!(unique.len(), keys.len(), "duplicate action key in {keys:?}");
    }

    #[test]
    fn prompt_body_survives_a_device_that_reports_nothing() {
        let body = prompt_body(&device("block"));
        assert!(!body.is_empty());
        assert_eq!(body.lines().next(), Some("device 1"));
    }
}
