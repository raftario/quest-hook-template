# quest-hook-template

A template for writing mods for Quest il2cpp games in Rust using [`quest-hook`](https://github.com/StackDoubleFlow/quest-hook-rs).

## Using this template

Simply use [`cargo generate`](https://github.com/cargo-generate/cargo-generate) to clone the template:

```
cargo generate --git https://github.com/raftario/quest-hook-template.git --name my-mod
```

Dont forget to [add an open source license](https://choosealicense.com/) so other people can use and contribute to your mod, and update this readme file.

## Prerequisites

To build mods, you will need:

- Rust. The easiest way to get it is using [rustup](https://rustup.rs).
- The Android NDK. Grab it from the [Android website](https://developer.android.com/ndk/downloads).
- An `ANDROID_NDK_HOME` environment variable poiting to the directory where your NDK is installed.
- `cargo ndk`. Install it with `cargo install cargo-ndk`.

## Development

- Use `cargo ndk build` to build the mod. Make sure to use `cargo ndk build --release` when releasing publically to turn on optimisations and keep everyone's framerate smooth.
- Use `cargo ndk clippy` to check for warnings or errors in your code. If you use VSCode with the rust-analyzer extension, this will be integrated in the editor.
- Use `cargo ndk doc --open` to generate interactive documentation for your mod and its dependencies and open it in your default browser.
