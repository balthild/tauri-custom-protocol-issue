// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use tauri::{
    http::{Request, Response, ResponseBuilder},
    AppHandle,
};

fn protocol_handle(_app: &AppHandle, req: &Request) -> Result<Response, Box<dyn Error>> {
    return ResponseBuilder::new()
        .body(format!("Greet from myproto:// handler. URL:\n{}", req.uri()).into());
}

fn main() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("myproto", protocol_handle)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
