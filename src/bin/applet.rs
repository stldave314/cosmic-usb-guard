// SPDX-License-Identifier: GPL-3.0-or-later

//! The `cosmic-usb-guard` panel applet.

fn main() -> cosmic::iced::Result {
    cosmic_usb_guard::init();
    cosmic_usb_guard::applet::run()
}
