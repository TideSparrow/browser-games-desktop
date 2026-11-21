#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{webview::NewWindowResponse, Url, WebviewUrl, WebviewWindowBuilder};

const HOME: &str = "https://browsergamehub.net/";

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let home = Url::parse(HOME)?;
            let origin = home.origin();

            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(home))
                .title("Browser Game Hub")
                .inner_size(1280.0, 850.0)
                .min_inner_size(460.0, 420.0)
                .on_navigation(move |url| url.scheme() == "https" && url.origin() == origin)
                .on_new_window(|_, _| NewWindowResponse::Deny)
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to start Browser Game Hub");
}
