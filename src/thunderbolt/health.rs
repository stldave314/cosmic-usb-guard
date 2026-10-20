// SPDX-License-Identifier: GPL-3.0-or-later

//! Whether Thunderbolt/USB4 is set up so a device cannot reach memory or the
//! PCIe bus without someone allowing it.
//!
//! Each check is a pure function of what was observed, so the rules for what
//! counts as a problem — and how bad — are tested without the hardware. Only
//! [`evaluate`] touches the system.

use std::path::Path;

use crate::constants::{BOLT_SERVICE_FILE, THUNDERBOLT_SYSFS};
use crate::usbguard::health::{Authorization, polkit_check};
use crate::usbguard::{Check, CheckId, Error, Steps};

use super::model::{SecurityLevel, Snapshot};
use super::sysfs::{self, KernelDomain};

/// Polkit action guarding enrollment, the stronger of bolt's two
/// authorization actions.
const ACTION_ENROLL: &str = "org.freedesktop.bolt.enroll";

/// Describe each domain's value of something, e.g. `domain0: user`.
fn describe(domains: &[KernelDomain], value: impl Fn(&KernelDomain) -> String) -> String {
    domains
        .iter()
        .map(|d| format!("{}: {}", d.name, value(d)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Whether devices need authorization before they get PCIe access.
///
/// `none` connects every device on sight. How bad that is depends on the
/// IOMMU: with DMA protection on, a device still cannot read arbitrary
/// memory, so it is a warning; without it, plugging something in is enough
/// to read the machine's memory, so it is critical.
pub fn security_check(domains: &[KernelDomain]) -> Check {
    let detail = describe(domains, |d| d.security.as_str().to_string());

    let open = domains
        .iter()
        .filter(|d| d.security.connects_without_asking());
    let mut worst = None;
    for domain in open {
        let unprotected = domain.dma_protection != Some(true);
        worst = Some(worst.unwrap_or(false) || unprotected);
    }

    match worst {
        Some(true) => Check::critical(CheckId::ThunderboltSecurity, detail, None)
            .with_steps(Steps::ThunderboltSecurity),
        Some(false) => Check::warn(CheckId::ThunderboltSecurity, detail, None)
            .with_steps(Steps::ThunderboltSecurity),
        None if domains
            .iter()
            .any(|d| matches!(d.security, SecurityLevel::Unknown(_))) =>
        {
            Check::warn(CheckId::ThunderboltSecurity, detail, None)
        }
        None => Check::ok(CheckId::ThunderboltSecurity, detail),
    }
}

/// Whether the IOMMU stops Thunderbolt devices reading arbitrary memory.
///
/// Not needed where the security level disables PCIe tunneling altogether:
/// with no PCIe there is no DMA to protect against.
pub fn dma_check(domains: &[KernelDomain]) -> Check {
    let needs = |d: &&KernelDomain| {
        !matches!(
            d.security,
            SecurityLevel::DpOnly | SecurityLevel::UsbOnly | SecurityLevel::NoPcie
        )
    };
    let detail = describe(domains, |d| match d.dma_protection {
        Some(true) => crate::fl!("observed-on"),
        Some(false) => crate::fl!("observed-off"),
        None => crate::fl!("observed-not-reported"),
    });

    if domains
        .iter()
        .filter(needs)
        .any(|d| d.dma_protection == Some(false))
    {
        Check::warn(CheckId::ThunderboltDmaProtection, detail, None)
            .with_steps(Steps::DmaProtection)
    } else if domains
        .iter()
        .filter(needs)
        .any(|d| d.dma_protection.is_none())
    {
        // An older kernel that does not say. Not evidence of a problem, but
        // not evidence of protection either.
        Check::warn(CheckId::ThunderboltDmaProtection, detail, None)
    } else {
        Check::ok(CheckId::ThunderboltDmaProtection, detail)
    }
}

/// The command that installs a package with this system's package manager.
///
/// `None` when none of the common ones is present; a guessed command for the
/// wrong distribution would be worse than none.
fn install_command(package: &str) -> Option<String> {
    let managers: [(&str, &str); 4] = [
        ("/usr/bin/apt", "sudo apt install"),
        ("/usr/bin/dnf", "sudo dnf install"),
        ("/usr/bin/zypper", "sudo zypper install"),
        ("/usr/bin/pacman", "sudo pacman -S"),
    ];
    managers
        .iter()
        .find(|(path, _)| Path::new(path).exists())
        .map(|(_, command)| format!("{command} {package}"))
}

/// Whether bolt answered.
///
/// Without bolt nothing authorizes Thunderbolt devices at the `user` and
/// `secure` levels: they stay unusable rather than unsafe, so this is a
/// warning. Two different fixes, depending on whether it is installed at all.
pub fn service_check(bolt: &Result<Snapshot, Error>, installed: bool) -> Check {
    match bolt {
        Ok(_) => Check::ok(CheckId::ThunderboltService, crate::fl!("observed-running")),
        Err(Error::ServiceUnavailable(_)) if !installed => Check::warn(
            CheckId::ThunderboltService,
            crate::fl!("observed-not-installed"),
            install_command("bolt"),
        ),
        Err(Error::ServiceUnavailable(_)) => Check::warn(
            CheckId::ThunderboltService,
            crate::fl!("observed-stopped"),
            Some("sudo systemctl unmask bolt.service && sudo systemctl start bolt.service".into()),
        ),
        Err(e) => Check::warn(CheckId::ThunderboltService, e.to_string(), None),
    }
}

/// Whether this user may authorize Thunderbolt devices.
///
/// Being asked for a password passes: proving who is at the keyboard before a
/// device gets PCIe access is bolt's own default, and a sound one, so the app
/// offers nothing to remove it. A refusal for the active local session is an
/// administrator's choice and is reported with the session steps but no rule
/// to override it; the usual cause is a session that is not the active local
/// one.
pub fn permission_check(authorization: Result<Authorization, String>) -> Check {
    let id = CheckId::ThunderboltPermission;
    match authorization {
        Ok(Authorization::Granted) => Check::ok(id, crate::fl!("observed-granted")),
        Ok(Authorization::Challenge) => Check::ok(id, crate::fl!("observed-asks-password")),
        Ok(Authorization::Refused) => {
            Check::warn(id, crate::fl!("observed-refused"), None).with_steps(Steps::InactiveSession)
        }
        Err(detail) => Check::warn(id, detail, None),
    }
}

/// Run every Thunderbolt check against the running system.
///
/// Returns nothing on a machine without Thunderbolt or USB4: there is no
/// attack surface to report on, and a page of "not applicable" rows would
/// only bury the checks that do apply.
pub async fn evaluate(
    bolt: &Result<Snapshot, Error>,
    connection: Option<&zbus::Connection>,
) -> Vec<Check> {
    let domains = sysfs::domains(Path::new(THUNDERBOLT_SYSFS));
    if domains.is_empty() {
        return Vec::new();
    }

    let mut checks = vec![
        security_check(&domains),
        dma_check(&domains),
        service_check(bolt, Path::new(BOLT_SERVICE_FILE).exists()),
    ];

    // Who may authorize only matters where devices are authorized one by
    // one, and can only be asked once bolt is there to do the authorizing.
    let asks = domains.iter().any(|d| d.security.asks_per_device());
    if asks
        && bolt.is_ok()
        && let Some(connection) = connection
    {
        checks.push(permission_check(
            polkit_check(connection, ACTION_ENROLL).await,
        ));
    }

    checks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usbguard::Severity;

    fn domain(security: &str, dma: Option<bool>) -> KernelDomain {
        KernelDomain {
            name: "domain0".into(),
            security: SecurityLevel::parse(security),
            dma_protection: dma,
        }
    }

    #[test]
    fn this_machines_configuration_passes() {
        // domain0: security `user`, iommu_dma_protection 1.
        let domains = [domain("user", Some(true))];
        assert_eq!(security_check(&domains).severity, Severity::Ok);
        assert_eq!(dma_check(&domains).severity, Severity::Ok);
        assert_eq!(security_check(&domains).detail, "domain0: user");
    }

    #[test]
    fn no_security_without_dma_protection_is_critical_and_says_how_to_fix_it() {
        let check = security_check(&[domain("none", Some(false))]);
        assert_eq!(check.severity, Severity::Critical);
        // The fix lives in firmware. A command here would be a lie.
        assert_eq!(check.remedy, None);
        assert_eq!(check.steps, Some(Steps::ThunderboltSecurity));
    }

    #[test]
    fn no_security_with_dma_protection_is_a_warning() {
        let check = security_check(&[domain("none", Some(true))]);
        assert_eq!(check.severity, Severity::Warning);
        assert_eq!(check.steps, Some(Steps::ThunderboltSecurity));
    }

    #[test]
    fn one_open_domain_is_enough_to_fail() {
        let mut second = domain("none", Some(false));
        second.name = "domain1".into();
        let check = security_check(&[domain("secure", Some(true)), second]);
        assert_eq!(check.severity, Severity::Critical);
        assert_eq!(check.detail, "domain0: secure, domain1: none");
    }

    #[test]
    fn levels_that_never_tunnel_pcie_pass_and_need_no_dma_protection() {
        for level in ["dponly", "usbonly", "nopcie"] {
            let domains = [domain(level, Some(false))];
            assert_eq!(security_check(&domains).severity, Severity::Ok, "{level}");
            assert_eq!(dma_check(&domains).severity, Severity::Ok, "{level}");
        }
    }

    #[test]
    fn dma_protection_off_points_at_firmware() {
        let check = dma_check(&[domain("user", Some(false))]);
        assert_eq!(check.severity, Severity::Warning);
        assert_eq!(check.steps, Some(Steps::DmaProtection));
    }

    #[test]
    fn an_unreported_dma_state_is_flagged_without_a_fix() {
        let check = dma_check(&[domain("user", None)]);
        assert_eq!(check.severity, Severity::Warning);
        assert_eq!(check.steps, None);
    }

    #[test]
    fn an_unknown_security_level_is_flagged_rather_than_trusted() {
        let check = security_check(&[domain("something-new", Some(true))]);
        assert_eq!(check.severity, Severity::Warning);
        assert_eq!(check.steps, None);
    }

    #[test]
    fn a_missing_bolt_gets_the_fix_that_matches_why() {
        let gone = Err(Error::ServiceUnavailable("no owner".into()));

        let stopped = service_check(&gone, true);
        assert_eq!(stopped.severity, Severity::Warning);
        assert!(
            stopped
                .remedy
                .unwrap()
                .contains("systemctl start bolt.service")
        );

        let absent = service_check(&gone, false);
        assert_eq!(absent.detail, "not installed");
        // The install command depends on the package manager present; when
        // there is one, it installs bolt.
        if let Some(remedy) = absent.remedy {
            assert!(
                remedy.starts_with("sudo ") && remedy.ends_with(" bolt"),
                "{remedy}"
            );
        }

        assert_eq!(
            service_check(&Ok(Snapshot::default()), true).severity,
            Severity::Ok
        );
    }

    #[test]
    fn being_asked_for_a_password_to_authorize_passes_and_nothing_is_offered() {
        let asks = permission_check(Ok(Authorization::Challenge));
        assert_eq!(asks.severity, Severity::Ok);
        assert_eq!(asks.remedy, None);
        assert_eq!(
            permission_check(Ok(Authorization::Granted)).severity,
            Severity::Ok
        );
        let refused = permission_check(Ok(Authorization::Refused));
        assert_eq!(refused.severity, Severity::Warning);
        assert_eq!(refused.remedy, None);
        assert_eq!(refused.steps, Some(Steps::InactiveSession));
    }
}
