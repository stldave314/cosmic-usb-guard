// SPDX-License-Identifier: GPL-3.0-or-later

//! Every icon name the UI uses must actually resolve.
//!
//! A missing icon renders as an empty square with no warning, which for a
//! panel indicator means an invisible applet and for the status page means a
//! severity the user cannot see. Guessing at plausible freedesktop names is
//! how that happens, so the names are checked against the installed themes.
//!
//! The test cannot pass vacuously: it first asserts that a control icon —
//! one every icon theme ships — is present. If the icon themes are not
//! installed, that assertion fails with an explanation rather than the test
//! quietly finding nothing to check.

use std::path::{Path, PathBuf};

use cosmic_usb_guard::ui::icons;

/// Directories searched for icon themes, in freedesktop order.
fn icon_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(&home).join(".local/share/icons"));
        roots.push(PathBuf::from(&home).join(".icons"));
    }

    let data_dirs = std::env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/local/share:/usr/share".to_string());
    for dir in data_dirs.split(':').filter(|d| !d.is_empty()) {
        roots.push(Path::new(dir).join("icons"));
    }
    roots.push(PathBuf::from("/usr/share/pixmaps"));

    roots.into_iter().filter(|root| root.is_dir()).collect()
}

/// Whether `name` resolves to a file in any installed theme.
///
/// Walks the theme trees rather than parsing `index.theme`: the question here
/// is only "does an icon by this name exist anywhere", which is exactly what
/// the toolkit's lookup will answer at runtime.
fn icon_exists(name: &str) -> bool {
    fn search(dir: &Path, name: &str, depth: usize) -> bool {
        if depth > 6 {
            return false;
        }
        let Ok(entries) = std::fs::read_dir(dir) else {
            return false;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                if search(&path, name, depth + 1) {
                    return true;
                }
            } else if let Some(stem) = path.file_stem().and_then(|s| s.to_str())
                && stem == name
                && matches!(
                    path.extension().and_then(|e| e.to_str()),
                    Some("svg" | "png" | "xpm")
                )
            {
                return true;
            }
        }
        false
    }

    icon_roots().iter().any(|root| search(root, name, 0))
}

/// Icons that every freedesktop icon theme ships. Used to prove the test has
/// a real fixture to work against.
const CONTROL_ICONS: &[&str] = &["dialog-warning-symbolic", "emblem-ok-symbolic"];

/// Every icon name the application can ask for.
fn all_icons() -> Vec<(&'static str, &'static str)> {
    vec![
        ("DEVICE", icons::DEVICE),
        ("STORAGE", icons::STORAGE),
        ("INPUT", icons::INPUT),
        ("NETWORK", icons::NETWORK),
        ("AUDIO", icons::AUDIO),
        ("CAMERA", icons::CAMERA),
        ("PRINTER", icons::PRINTER),
        ("OK", icons::OK),
        ("WARNING", icons::WARNING),
        ("ERROR", icons::ERROR),
        ("QUESTION", icons::QUESTION),
        ("BLOCKED", icons::BLOCKED),
        ("REMOVE", icons::REMOVE),
        ("COPY", icons::COPY),
        ("REFRESH", icons::REFRESH),
        ("QUIT", icons::QUIT),
        ("HOOK", icons::HOOK),
        ("HISTORY", icons::HISTORY),
        ("SETTINGS", icons::SETTINGS),
        ("PANEL_OK", icons::PANEL_OK),
        ("PANEL_WARNING", icons::PANEL_WARNING),
        ("PANEL_CRITICAL", icons::PANEL_CRITICAL),
    ]
}

#[test]
fn icon_themes_are_installed() {
    // The fixture check. Without this, `every_icon_name_resolves` would pass
    // on a machine with no icons at all.
    assert!(
        !icon_roots().is_empty(),
        "no icon theme directories found; install an icon theme \
         (e.g. adwaita-icon-theme) before running the test suite"
    );

    for control in CONTROL_ICONS {
        assert!(
            icon_exists(control),
            "control icon `{control}` is missing, so this suite cannot tell a \
             wrong icon name from an uninstalled theme; install adwaita-icon-theme"
        );
    }
}

#[test]
fn every_icon_name_resolves() {
    // Re-assert the fixture so this test cannot pass on an empty system even
    // when run alone with `--exact`.
    assert!(
        CONTROL_ICONS.iter().all(|name| icon_exists(name)),
        "icon themes are not installed; see `icon_themes_are_installed`"
    );

    let missing: Vec<&str> = all_icons()
        .into_iter()
        .filter(|(_, name)| !icon_exists(name))
        .map(|(constant, name)| Box::leak(format!("{constant} = {name}").into_boxed_str()) as &str)
        .collect();

    assert!(
        missing.is_empty(),
        "these icon names do not resolve and would render as blank squares: {missing:#?}"
    );
}

/// Icon themes we treat as the portability baseline.
///
/// `every_icon_name_resolves` searches every installed theme, which on a
/// Pop!_OS or GNOME machine includes names that exist nowhere else. That is
/// how `audio-card-usb-symbolic` passed locally and failed on a stock runner.
/// Checking against a baseline theme catches it before it is pushed.
const BASELINE_THEMES: &[&str] = &["Adwaita", "hicolor"];

fn baseline_roots() -> Vec<PathBuf> {
    icon_roots()
        .iter()
        .flat_map(|root| BASELINE_THEMES.iter().map(move |theme| root.join(theme)))
        .filter(|path| path.is_dir())
        .collect()
}

fn resolves_in_baseline(name: &str) -> bool {
    fn search(dir: &Path, name: &str, depth: usize) -> bool {
        if depth > 6 {
            return false;
        }
        let Ok(entries) = std::fs::read_dir(dir) else {
            return false;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                if search(&path, name, depth + 1) {
                    return true;
                }
            } else if path.file_stem().and_then(|s| s.to_str()) == Some(name)
                && matches!(
                    path.extension().and_then(|e| e.to_str()),
                    Some("svg" | "png" | "xpm")
                )
            {
                return true;
            }
        }
        false
    }

    baseline_roots().iter().any(|root| search(root, name, 0))
}

#[test]
fn every_icon_name_resolves_in_a_baseline_theme() {
    // Fixture guard: without a baseline theme this would check nothing.
    assert!(
        !baseline_roots().is_empty(),
        "no baseline icon theme found (looked for {BASELINE_THEMES:?}); \
         install adwaita-icon-theme before running the test suite"
    );
    assert!(
        resolves_in_baseline("dialog-warning-symbolic"),
        "the baseline theme is present but does not contain a control icon, \
         so this check could not detect a non-portable name"
    );

    let missing: Vec<String> = all_icons()
        .into_iter()
        .filter(|(_, name)| !resolves_in_baseline(name))
        .map(|(constant, name)| format!("{constant} = {name}"))
        .collect();

    assert!(
        missing.is_empty(),
        "these icon names exist only in a theme this machine happens to have, \
         and would render as blank squares elsewhere: {missing:#?}"
    );
}

#[test]
fn panel_icons_are_distinct() {
    // The panel icon is the only protection signal a user sees without
    // opening anything; three states that render identically would be worse
    // than no indicator.
    assert_ne!(icons::PANEL_OK, icons::PANEL_WARNING);
    assert_ne!(icons::PANEL_WARNING, icons::PANEL_CRITICAL);
    assert_ne!(icons::PANEL_OK, icons::PANEL_CRITICAL);
}
