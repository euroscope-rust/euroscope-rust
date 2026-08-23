//! Build script for `euroscope-sys`.
//!
//! What this does:
//! - locate the EuroScope plugin "SDK" (header and lib file)
//! - compile the C++ glue shim (`src/shim/`) against that header //, targeting 32-bit x86
//!   (EuroScope is a 32-bit MFC app)
//! - link the shim into downstream crates with `+whole-archive` so the `__declspec(dllexport)`
//!   entry points (`EuroScopePlugInInit` / `EuroScopePlugInExit`) survive into the final `cdylib`
//!   even though no Rust code references them directly
//! - link the EuroScope import library so the shim's calls into `EuroScope.exe` resolve

use std::{env::var, path::PathBuf};

const EUROSCOPE_PLUGIN_HEADER_NAME: &str = "EuroScopePlugIn.h";
const EUROSCOPE_PLUGIN_LIB_NAME: &str = "EuroScopePlugInDll.lib";

/// Find the EuroScope plugin "SDK" dir (where header and lib live).
///
/// Prority for discovery:
/// - `EUROSCOPE_PLUGIN_SDK_DIR` env var: a dir where both header and lib live
/// - `%APPDATA%/EuroScope/PlugIn`: the SDK should be installed there during the default EuroScope
///   installation
///
/// The header must be named as specified in [`EUROSCOPE_PLUGIN_HEADER_NAME`]
/// and the lib must be named as specified in [`EUROSCOPE_PLUGIN_LIB_NAME`].
fn locate_sdk() -> PathBuf {
    println!("cargo:rerun-if-env-changed=EUROSCOPE_PLUGIN_SDK_DIR");

    let dir = var("EUROSCOPE_PLUGIN_SDK_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let appdata =
                var("APPDATA").expect("APPDATA is not set; cannot locate EuroScope plugin SDK lib");
            PathBuf::from(appdata).join("EuroScope").join("PlugIn")
        });

    if !dir.join(EUROSCOPE_PLUGIN_HEADER_NAME).is_file() {
        panic!(
            "The directory `{}` did not contain a header file named `{}`",
            dir.display(),
            EUROSCOPE_PLUGIN_HEADER_NAME,
        );
    }

    if !dir.join(EUROSCOPE_PLUGIN_LIB_NAME).is_file() {
        panic!(
            "The directory `{}` did not contain a header file named `{}`",
            dir.display(),
            EUROSCOPE_PLUGIN_LIB_NAME,
        );
    }

    dir
}

fn main() {
    // Check we're compiling against the correct target
    let target = var("TARGET").unwrap_or_default();
    if !target.contains("windows-msvc") {
        panic!(
            "euroscope-sys only supports *-windows-msvc targets (got `{target}`). EuroScope \
             plugins must be built with the MSVC toolchain."
        );
    }
    if !target.starts_with("i686") {
        println!(
            "cargo:warning=Target is `{target}`, but EuroScope is 32-bit. Build with --target \
             i686-pc-windows-msvc."
        );
    }

    println!("cargo:rerun-if-env-changed=DOCS_RS");
    if var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-changed=src/shim");

    let manifest_dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());
    let sdk_dir = locate_sdk();

    // Compile the C++ shim. `cc` auto-selects the 32-bit MSVC compiler from TARGET
    // and sets up the Windows SDK include paths that `EuroScopePlugIn.h` needs
    // (it pulls in <windows.h> for RECT/HDC/POINT).
    // Every .cpp under src/shim/ is a shim translation unit; collect them so
    // adding a new per-class wrapper file needs no build.rs change.
    let shim_dir = manifest_dir.join("src").join("shim");
    let mut sources: Vec<PathBuf> = std::fs::read_dir(&shim_dir)
        .expect("src/shim directory")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "cpp"))
        .collect();
    sources.sort();

    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .files(&sources)
        .include(&sdk_dir)
        .include(&shim_dir)
        // EuroScope + this SDK are compiled with the multi-byte (ANSI) charset,
        // never Unicode. Match it so `char*` APIs line up.
        .define("_MBCS", None)
        .cargo_metadata(false)
        .warnings(false)
        // Force the dynamic *release* CRT. rustc always links `/MD` (there is
        // no debug CRT in Rust std), but `cc` picks `/MDd` for debug profiles;
        // mixing `/MDd` and `/MD` in one image corrupts `new`/`delete` across
        // the boundary.  Passing `/MD` last makes it win (cl emits a harmless
        // D9025 override).
        .flag("/MD")
        .compile("euroscope_shim");

    let out_dir = var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={out_dir}");
    // whole-archive: force every object (including the unreferenced exports)
    // into the final DLL.
    println!("cargo:rustc-link-lib=static:+whole-archive=euroscope_shim");

    // The EuroScope import library: satisfies the shim's calls into
    // EuroScope.exe.
    println!("cargo:rustc-link-search=native={}", sdk_dir.display());
    println!("cargo:rustc-link-lib=EuroScopePlugInDll");
}
