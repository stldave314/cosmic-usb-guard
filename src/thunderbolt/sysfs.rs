// SPDX-License-Identifier: GPL-3.0-or-later

//! The kernel's own view of Thunderbolt/USB4, read from sysfs.
//!
//! This is the authority for the two settings that decide whether a device can
//! reach memory: the domain's security level and whether the IOMMU protects
//! against DMA. Both are readable by any user and need no running service, so
//! the health checks still work when bolt is missing — which is exactly when
//! they matter.

use std::path::Path;

use super::model::SecurityLevel;

/// A Thunderbolt domain as the kernel reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelDomain {
    /// Directory name, e.g. `domain0`.
    pub name: String,
    /// The `security` attribute.
    pub security: SecurityLevel,
    /// The `iommu_dma_protection` attribute; `None` on kernels too old to
    /// report it.
    pub dma_protection: Option<bool>,
}

/// Every Thunderbolt domain under `root`, sorted by name.
///
/// `root` is [`crate::constants::THUNDERBOLT_SYSFS`] outside tests. An empty
/// result means the machine has no Thunderbolt or USB4 controller, or the
/// `thunderbolt` driver is not loaded — either way there is nothing to check.
pub fn domains(root: &Path) -> Vec<KernelDomain> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return Vec::new();
    };

    let mut domains: Vec<KernelDomain> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().to_str()?.to_string();
            if !name.starts_with("domain") {
                return None;
            }
            let path = entry.path();
            let security = std::fs::read_to_string(path.join("security")).ok()?;
            let dma_protection = std::fs::read_to_string(path.join("iommu_dma_protection"))
                .ok()
                .map(|value| value.trim() == "1");
            Some(KernelDomain {
                name,
                security: SecurityLevel::parse(&security),
                dma_protection,
            })
        })
        .collect();

    domains.sort_by(|a, b| a.name.cmp(&b.name));
    domains
}

/// Whether this machine has a Thunderbolt or USB4 controller to manage.
pub fn present() -> bool {
    !domains(Path::new(crate::constants::THUNDERBOLT_SYSFS)).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway sysfs tree.
    struct Tree(std::path::PathBuf);

    impl Tree {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "cosmic-usb-guard-sysfs-{name}-{}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(&root).unwrap();
            Self(root)
        }

        fn domain(&self, name: &str, security: &str, dma: Option<&str>) -> &Self {
            let dir = self.0.join(name);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("security"), format!("{security}\n")).unwrap();
            if let Some(dma) = dma {
                std::fs::write(dir.join("iommu_dma_protection"), format!("{dma}\n")).unwrap();
            }
            self
        }
    }

    impl Drop for Tree {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn reads_domains_and_ignores_the_devices_beside_them() {
        let tree = Tree::new("read");
        tree.domain("domain1", "none", Some("0"))
            .domain("domain0", "user", Some("1"));
        // Routers and ports live in the same directory and are not domains.
        std::fs::create_dir_all(tree.0.join("0-0")).unwrap();

        let found = domains(&tree.0);
        assert_eq!(
            found,
            vec![
                KernelDomain {
                    name: "domain0".into(),
                    security: SecurityLevel::User,
                    dma_protection: Some(true),
                },
                KernelDomain {
                    name: "domain1".into(),
                    security: SecurityLevel::None,
                    dma_protection: Some(false),
                },
            ]
        );
    }

    #[test]
    fn an_old_kernel_without_dma_reporting_is_unknown_not_unprotected() {
        let tree = Tree::new("old");
        tree.domain("domain0", "secure", None);
        assert_eq!(domains(&tree.0)[0].dma_protection, None);
    }

    #[test]
    fn a_machine_without_thunderbolt_has_no_domains() {
        assert!(domains(Path::new("/nonexistent/thunderbolt")).is_empty());
    }
}
