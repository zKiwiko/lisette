mod project_manifest;
mod typedef_locator;

use std::path::{Path, PathBuf};

pub use project_manifest::{
    GoDependency, Manifest, check_toolchain_version, parse_manifest, remove_go_dep_from_manifest,
    write_go_dep_to_manifest,
};
pub use typedef_locator::{TypedefLocator, TypedefLocatorResult, TypedefOrigin};

pub fn is_third_party(pkg: &str) -> bool {
    pkg.split('/')
        .next()
        .is_some_and(|first| first.contains('.'))
}

pub fn is_stdlib(pkg: &str) -> bool {
    !is_third_party(pkg)
}

pub fn typedef_cache_dir(home: &str) -> PathBuf {
    let lis_version = env!("CARGO_PKG_VERSION");
    PathBuf::from(home).join(format!(".lisette/cache/typedefs/lis@v{}", lis_version))
}

/// A Go package within a versioned module.
pub struct GoPackageRef<'a> {
    /// Module path, e.g. `github.com/gorilla/mux`.
    pub module_path: &'a str,
    /// Module version, e.g. `v1.8.0`.
    pub version: &'a str,
    /// Package import path, either identical to `module_path` for the root package,
    /// or extended for subpackages (e.g. `github.com/gorilla/mux/middleware`).
    pub package_path: &'a str,
}

impl GoPackageRef<'_> {
    /// Build the path to a `.d.lis` file under a base directory.
    ///
    /// ```text
    /// ~/.lisette/cache/typedefs/lis@v0.1.6/github.com/gorilla/mux@v1.8.0/mux.d.lis
    /// ~/.lisette/cache/typedefs/lis@v0.1.6/github.com/gorilla/mux@v1.8.0/middleware/middleware.d.lis
    /// ```
    pub fn build_typedef_path(&self, base: &Path) -> PathBuf {
        let module_dir = base.join(format!("{}@{}", self.module_path, self.version));

        let relative = if self.package_path == self.module_path {
            ""
        } else {
            self.package_path
                .strip_prefix(self.module_path)
                .and_then(|s| s.strip_prefix('/'))
                .unwrap_or("")
        };

        let last_segment = self
            .package_path
            .rsplit('/')
            .next()
            .unwrap_or(self.package_path);

        let filename = format!("{}.d.lis", last_segment);

        if relative.is_empty() {
            module_dir.join(filename)
        } else {
            module_dir.join(relative).join(&filename)
        }
    }
}
