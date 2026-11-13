//! Detect whether the compiler's `Allocator` API is available on stable
//! (Rust 1.100+) so we only enable `#![feature(allocator_api)]` on older
//! nightlies that still need the gate.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Silence unexpected_cfgs on toolchains that understand check-cfg.
    println!("cargo:rustc-check-cfg=cfg(allocator_api_stable)");

    if rustc_at_least(1, 100) {
        println!("cargo:rustc-cfg=allocator_api_stable");
    }
}

fn rustc_at_least(major: u32, minor: u32) -> bool {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = std::process::Command::new(rustc)
        .arg("--version")
        .output()
        .expect("failed to run `rustc --version`");
    if !output.status.success() {
        panic!(
            "`rustc --version` failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let version = String::from_utf8(output.stdout).expect("rustc --version was not utf-8");
    let (v_major, v_minor) = parse_rustc_version(&version)
        .unwrap_or_else(|| panic!("failed to parse rustc version from: {version:?}"));
    (v_major, v_minor) >= (major, minor)
}

/// Parse `rustc 1.100.0 (...)` / `rustc 1.102.0-nightly (...)` into (major, minor).
fn parse_rustc_version(version: &str) -> Option<(u32, u32)> {
    let mut parts = version.split_whitespace();
    if parts.next()? != "rustc" {
        return None;
    }
    let semver = parts.next()?;
    let mut nums = semver.split(|c: char| !c.is_ascii_digit());
    let major = nums.next()?.parse().ok()?;
    let minor = nums.next()?.parse().ok()?;
    Some((major, minor))
}
