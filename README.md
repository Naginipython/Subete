# Omniyomi

A personal project attempting to create a Tachiyomi/Aniyomi-like for desktop from scratch.

Currently only uses a MangaDex extention, but more can easily be added and should simply work.

To-Build:\
Install node & rust (rustup)\
In a command line, install tauri-cli: `cargo install tauri-cli`\
In the `Omniyomi` folder, run: `cargo tauri dev` for testing, `cargo tauri build` to build release (will be in `target/release/bundle/`).\
Note: On Linux, release build may need to be `NO_STRIP=true cargo tauri build`

NOTE: Some pages randomly don't work first run, finding the page in `frontend/src/routes` and pressing save helps reload it\
Currently, mostly `frontent/src/routes/library/+page.svelte` has this issue