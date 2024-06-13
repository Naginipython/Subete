# Omniyomi

A personal project attempting to create a Tachiyomi/Aniyomi-like for desktop from scratch. Currently features manga reading, library, and app setting modification, soon to be implementing Light Novel support

Currently only has a built-in MangaDex plugin, but features a plugin creator in `plugins/` to help creating plugins for this app. Due to limitation on how JavaScript code runs within the Rust code, do not rely on npm packages unless they are hard-coded, and avoid odd symbols because of JSON parsing (e.g. \\\\. \\\' seems fine)

To-Build Pt.1:\
Install node & rust (rustup) (Note: on Manjaro Linux, I also needed webkit2gtk)\
In a command line, install tauri-cli: `cargo install tauri-cli@^2.0.0-beta` (note: may need quotation)\
Move to the `/frontend` folder, and run `npm i` or `npm install`\

To-Build Pt.2: Linux\
In the root folder, run: `cargo tauri dev` for testing, `cargo tauri build` to build release (will be in `target/release/bundle/`).\
Note: Release build may need to be `NO_STRIP=true cargo tauri build`

To-Build Pt.2: Windows\
In the root folder, run: `cargo tauri dev --target x86_64-pc-windows-msvc` for testing, `cargo tauri build --target x86_64-pc-windows-msvc` to build release (will be in `target/release/bundle/`).\
Optionally, go into `.cargo` and change `target = ` to `x86_64-pc-windows-msvc` so that you can run the same commands as Linux does\

NOTE: Some pages randomly don't work first run, finding the page in `frontend/src/routes` and pressing save helps reload it\
Currently, mostly `frontent/src/routes/library/+page.svelte` has this issue often

Android version does exist with `cargo tauri android dev`, however, I haven't used it yet due to other areas of focus and cannot attest to it's quality