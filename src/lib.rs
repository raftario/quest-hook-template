use quest_hook::{hook, libil2cpp::Il2CppObject};
use tracing_android::tracing::{debug, info};

// We uses procedural macros to generate the hook's glue code at compile time
// which lets us keep the syntax nice and clean, and leaves less rooms for typos
#[hook("", "MainSettingsModelSO", "OnEnable")]
fn on_enable(this: &mut Il2CppObject) {
    info!("Hello, World!");

    // The logging macros can format anything that implements the required traits
    debug!("The class of `this` is {}", this.class());

    // Calls to the original C# method use the previously generated glue code
    on_enable.original(this);
}

#[no_mangle]
pub extern "C" fn load() {
    // Setup a subscriber to send our logs to Android
    // Here we setup the tag to be the same as the package name from Cargo.toml,
    // but we could customise our logger as much as we want
    tracing_android::subscriber(env!("CARGO_PKG_NAME")).init();

    // Hooks are always type checked at installation
    // The installation fails if the method signatures don't match
    debug!("Installing hook for MainSettingsModelSO.OnEnable...");
    on_enable.install();

    debug!("Installed hooks!");
}
