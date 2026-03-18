// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::semicolon_if_nothing_returned)]

fn main() {
    kd_music_tool_lib::run()
}
