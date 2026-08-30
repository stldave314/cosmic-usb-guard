// SPDX-License-Identifier: GPL-3.0-or-later

//! The `cosmic-usb-guard` window application.

fn main() -> cosmic::iced::Result {
    cosmic_usb_guard::init();
    cosmic_usb_guard::app::run()
}
