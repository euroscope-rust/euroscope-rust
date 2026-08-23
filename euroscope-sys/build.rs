//! Build script for `euroscope-sys`.
//!
//! What this does:
//! - locate the EuroScope plugin "SDK" (header and lib file)
//! - compile the C++ glue shim (`src/shim/`) against that header //, targeting 32-bit x86
//!   (EuroScope is a 32-bit MFC app)
//! - link the shim into downstream crates, letting the usual static-library rules decide which
//!   objects come along; `register_plugin!` references `es_shim_anchor` to pull in the one holding
//!   the `__declspec(dllexport)` entry points, which no Rust code calls directly
//! - link the EuroScope import library so the shim's calls into `EuroScope.exe` resolve

use std::{
    env::{var, var_os},
    path::PathBuf,
};

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
    println!("cargo:rerun-if-env-changed=EUROSCOPE_PLUGIN_DELAYLOAD");
    if var_os("EUROSCOPE_PLUGIN_DELAYLOAD").is_some() {
        // Delay-loading the DLL defers resolving its imports until the first
        // call into EuroScope. The tests never call into EuroScope, so the DLL
        // is never loaded and the harness runs normally.
        // `delayimp.lib` provides the delay-load helper (`__delayLoadHelper2`);
        // `/DELAYLOAD` marks the EuroScope import so it resolves lazily.
        println!("cargo::rustc-link-arg=/DELAYLOAD:EuroScopePlugInDll.dll");
        println!("cargo::rustc-link-arg=delayimp.lib");
        // A test binary that happens to pull in no shim object at all imports
        // nothing from EuroScope, and the linker warns that /DELAYLOAD had
        // nothing to do (LNK4199). That is the expected case here, and CI
        // builds tests with `-Dwarnings`.
        println!("cargo::rustc-link-arg=/IGNORE:4199");
    }

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
    // Deliberately *not* `+whole-archive`. Forcing every object in would drag
    // `core.cpp` and `radar_screen.cpp` into artifacts that never act as a
    // plugin -- notably this workspace's own test harnesses -- leaving their
    // calls to the `register_plugin!`-provided `rust_*` callbacks unresolved.
    // `es_shim_anchor` gets those objects into real plugins instead.
    println!("cargo:rustc-link-lib=static=euroscope_shim");

    // The EuroScope import library: satisfies the shim's calls into
    // EuroScope.exe.
    println!("cargo:rustc-link-search=native={}", sdk_dir.display());
    println!("cargo:rustc-link-lib=EuroScopePlugInDll");
}
