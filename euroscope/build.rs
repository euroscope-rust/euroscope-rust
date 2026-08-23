fn main() {
    println!("cargo:rerun-if-env-changed=EUROSCOPE_PLUGIN_DELAYLOAD");
    if std::env::var_os("EUROSCOPE_PLUGIN_DELAYLOAD").is_some() {
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
}
