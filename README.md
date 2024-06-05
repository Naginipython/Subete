# Omniyomi

A personal project attempting to create a Tachiyomi/Aniyomi-like for desktop from scratch.

Currently only uses a MangaDex extention, but more can easily be added and should simply work.

NOTE: When first running this, select CSS will not work. I am unaware if it is a Svelte, Tauri, Webkit-something or personal issue. As of 5/31/24 running `cargo tauri dev` but then pressing `ctrl-s` on library's `+page.svelte` solves this. This can easily change to another page later