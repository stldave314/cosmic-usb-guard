// SPDX-License-Identifier: GPL-3.0-or-later

//! Localization.
//!
//! Every user-facing string goes through [`fl!`]. Fluent falls back silently
//! to the fallback language for a missing key, so a key that exists in `en`
//! and nowhere else shows up as stray English at runtime rather than as a
//! build error — which is why `tests/i18n.rs` asserts that every locale
//! carries exactly the same key set.

use std::sync::LazyLock;

use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::{DefaultLocalizer, LanguageLoader, Localizer};
use rust_embed::RustEmbed;

/// The `i18n/` directory, embedded into the binary.
#[derive(RustEmbed)]
#[folder = "i18n/"]
pub struct Localizations;

/// The shared Fluent loader.
pub static LANGUAGE_LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader = fluent_language_loader!();
    loader
        .load_fallback_language(&Localizations)
        .expect("i18n/en must be present and valid; it is embedded at build time");
    loader
});

/// Look up a localized string by key.
///
/// ```ignore
/// fl!("device-allowed");
/// fl!("devices-count", count = 3);
/// ```
#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

/// Localizer over the embedded assets.
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

/// Select the best available translation for the desktop's requested
/// languages.
///
/// A failure here is not fatal — the fallback language is already loaded — so
/// it is reported and execution continues.
pub fn init() {
    let localizer = localizer();
    if let Err(e) = localizer.select(&i18n_embed::DesktopLanguageRequester::requested_languages()) {
        crate::error_log!(crate::debug::CONFIG, "could not select a locale: {e}");
    }
}
