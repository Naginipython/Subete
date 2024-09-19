# Subete

A personal project attempting to create a ~~Tachiyomi~~ Mihon/Aniyomi-like for desktop from scratch. Currently features manga reading, library, and app setting modification, soon to be implementing Light Novel support (This may not work)

Currently only has a built-in MangaDex plugin, but features a plugin creator in `https://github.com/naginipython/subete_plugin_creator` to help creating plugins for this app. Due to limitation on how JavaScript code runs within the Rust code, do not rely on npm packages unless they are hard-coded, and avoid odd symbols because of JSON parsing (e.g. \\\\. \\\' seems fine)

Video featuring major features as of 06/17/24:

https://github.com/Naginipython/Subete/assets/42967504/260dfcf2-4acc-40cb-b75d-c54c9afe8216


## Goals
- [ ] Fix and doc Windows
- [ ] Make look professional
- [ ] History
- [x] Error messages for browse
- [x] Error messages for manga chapters.
- [ ] Error messages for ln chapters.
- [x] Error messages for anime episodes
- [ ] Error messages for readers
- [ ] Easier Plugin DOM parsing
- [x] Updates. Cute progress bar for updates
  - [ ] fix updates back button
  - [x] anime & ln
- [x] General loading icons for pages, browse.
- [x] App initialize loading icon
- [ ] Icons for nav bar
- [ ] nav bar more items when able
- [x] Hide Navbar when none selected (aka, manga/settings screen)
- [ ] Run app in background (& Auto-update)
- [ ] Update result notifications
- [ ] Fullscreen on Android for Reader (maybe not possible)
- [ ] Extension General Feed (Front page, if available)
- [ ] Settings:
  - [ ] Disable Manga/Ln/Anime
  - [ ] Show per row OR as many as fit
  - [ ] SPECIAL mode
  - [ ] Mode: tab above for M/L/A, OR Tabs for each lib
    - [ ] Browse & Update separation needed
  - [ ] Tab ordering
  - [ ] Repo link for extentions (like Tachiyomi)
- [ ] Library Features:
  - [ ] Search
  - [ ] Search plugin
  - [ ] Filter (completed, started, unseed)
  - [ ] Sort (alphabetically, total chap, last seen, last updated, unseen #, date added)
  - [ ] Setting: Togglable: Set # per row
  - [ ] Categories
  - [ ] Open Random
- [ ] Manga/Ln features:
  - [ ] Tracking (MyAnimeList)
  - [ ] Bookmark
  - [ ] Download (DEAD for android, unless I find workaround for tauri fs)
  - [ ] Select
  - [ ] Check down (all below)
  - [ ] Sort Asc or Desc
  - [ ] Filter: Unread, Downloaded, Bookmarked
  - [ ] Settings: Sort & Filter per item, button to set as global setting
  - [ ] Send to Web
  - [ ] Title hold sends to clipboard
  - [ ] Fix Description
  - [ ] Categories
  - [ ] fav button to start/resume (Topmost non-completed item)
- [ ] Manga Reader features:
  - [ ] Swipe turn
  - [ ] Bookmark
  - [ ] Cuter bottom page changer
  - [ ] Cuter Prev/Next screen
  - [ ] Open in Web
  - [ ] Next/Prev chap arrow
  - [ ] Page read type:  l-to-r, r-to-l, strip
  - [ ] Zoom
  - [ ] Zoom into large image toggle
  - [ ] tap area size change?
  - [ ] Jpn to English help
- [ ] LN Reader features:
  - [ ] progress (scrollTop %?)
  - [ ] next/prev chapter
  - [ ] bookmark
  - [ ] open in web
  - [ ] drag speed
  - [ ] top areas
  - [ ] next/prev chapter
- [ ] Anime
  - [ ] Choose linux watcher
  - [ ] Download

## Installation

To-Build Pt.1:\
Install all needed by [Tauri](https://beta.tauri.app/start/prerequisites/)\
In a command line, install tauri-cli: `cargo install tauri-cli@^2.0.0-rc` (note: may need quotation) you can also use other methods listed on [tauri's website](https://v2.tauri.app/reference/cli/)\
In the root folder, run `npm i` or `npm install`

To-Build Pt.2: Linux\
In the root folder, run: `cargo tauri dev` for testing, `cargo tauri build` to build release (will be in `target/release/bundle/`).\
Note: Release build may need to be `NO_STRIP=true cargo tauri build`

To-Build Pt.2: Windows\
In windows, Microsoft's C compiler won't work. You will need to use the `stable-x86_64-pc-windows-gnu` rust toolchain, which uses gcc. Install it with `rustup toolchain install stable-x86_64-pc-windows-gnu`, then you have 3 ways to use this: (NOTE: you may need msys2 gcc & enviroment variable for `window-gnu`)
1) In the root folder, run: `cargo tauri dev --target stable-x86_64-pc-windows-gnu` for testing, `cargo tauri build --target stable-x86_64-pc-windows-gnu` to build release (will be in `target/release/bundle/`).
2) `rustup default stable-x86_64-pc-windows-gnu` to set as your primary toolchain.
3) Create a `.cargo` folder, with a `config.toml` file. Add the line `[build]` at the top, and `target = "stable-x86_64-pc-windows-gnu"` below it.

As a note for newer people, the usual toolchain is `x86_64-pc-windows-msvc`.

NOTE: Some pages randomly don't work first run, finding the page in `src/routes`, cutting it all, save, then paste, save helps reload it\
Currently, mostly `src/routes/snackbar.svelte` has this issue often. (This is due to root styles)

## Android

Ideally, ensure that the standard installation is completed.\
In order to build to Android, download Android Studios, and the NDK from the Android Studio installer.\
Android Studio will have 3 variables to set up, one being the JBR as JAVA_HOME (located where it is installed `./jbr`. example: `export JAVA_HOME=/opt/android-studio/jbr` (Flatpak users RIP)), the SDK as ANDROID_SDK (`export ANDROID_HOME=$HOME/Android/Sdk`), and the NDK as NDK_HOME (`export NDK_HOME=$HOME/Android/Sdk/ndk/27.0.11902837`).\
Add Android targets with `rustup`: `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`\
In the main directory, run `cargo tauri android init`. Ideally, it will say that it is good to go.\
Connect your phone to the computer with `android-tools` (adb), set your phone's USB debugging on, then run `cargo tauri android dev` for a quick view of the app (Note: The frontend is locally created, so the app will be installed, but won't work)\
To build the app in an APK, use `cargo tauri android build`, going to where the APK is (says in console), signing it with `uber-apk-signer`, then installing it on my phone.
