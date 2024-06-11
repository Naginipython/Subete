# Omniyomi

A personal project attempting to create a Tachiyomi/Aniyomi-like for desktop from scratch. Currently features manga reading, library, and app setting modification, soon to be implementing Light Novel support

Currently only has a built-in MangaDex plugin, but features a plugin creator in `plugins/` to help creating plugins for this app. Due to limitation on how JavaScript code runs within the Rust code, do not rely on npm packages unless they are hard-coded, and avoid odd symbols because of JSON parsing (e.g. \\\\. \\\' seems fine)

To-Build:\
Install node & rust (rustup) (Note: on Manjaro Linux, I also needed webkit2gtk)\
In a command line, install tauri-cli: `cargo install tauri-cli@^2.0.0-beta` (note: may need quotation)\
In the `Omniyomi` folder, run: `cargo tauri dev` for testing, `cargo tauri build` to build release (will be in `target/release/bundle/`).\
Note: On Linux, release build may need to be `NO_STRIP=true cargo tauri build`

NOTE: Some pages randomly don't work first run, finding the page in `frontend/src/routes` and pressing save helps reload it\
Currently, mostly `frontent/src/routes/library/+page.svelte` has this issue often

Android version does exist with `cargo tauri android dev`, however, I haven't used it yet due to other areas of focus and cannot attest to it's quality