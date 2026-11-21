#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
use tauri::webview::PageLoadEvent;
use tauri::{webview::NewWindowResponse, Url, WebviewUrl, WebviewWindowBuilder};

const HOME: &str = "https://browsergamehub.net/";

#[cfg(target_os = "macos")]
fn is_game_host(url: &Url) -> bool {
    url.port().is_none()
        && matches!(
            url.host_str(),
            Some("html5.gamemonetize.co" | "deadlydescent.net" | "blockblastmaster.io")
        )
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let home = Url::parse(HOME)?;
            let origin = home.origin();

            let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(home.clone()))
                .title("Browser Game Hub")
                .inner_size(1280.0, 850.0)
                .min_inner_size(460.0, 420.0)
                .on_navigation(move |url| {
                    if url.scheme() != "https" {
                        return false;
                    }
                    #[cfg(target_os = "macos")]
                    if is_game_host(url) {
                        return true;
                    }
                    url.origin() == origin
                })
                .on_new_window(|_, _| NewWindowResponse::Deny);

            // WKWebView applies navigation policy to iframes as well as the main page.
            // Game hosts may load in iframes, while the main page stays on Browser Game Hub.
            #[cfg(target_os = "macos")]
            let window = window.on_page_load(move |window, payload| {
                if matches!(payload.event(), PageLoadEvent::Started)
                    && payload.url().origin() != home.origin()
                {
                    let _ = window.navigate(home.clone());
                }
            });
            window.build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to start Browser Game Hub");
}
