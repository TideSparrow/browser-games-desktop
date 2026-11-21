#[cfg(any(target_os = "macos", target_os = "android"))]
use tauri::webview::PageLoadEvent;
use tauri::{webview::NewWindowResponse, Url, WebviewUrl, WebviewWindowBuilder};

const HOME: &str = "https://browsergamehub.net/";
#[cfg(target_os = "android")]
const CAMPAIGN: &str = "browser-games-android-app";
#[cfg(not(target_os = "android"))]
const CAMPAIGN: &str = "browser-games-desktop-app";

#[cfg(any(target_os = "macos", target_os = "android"))]
fn is_game_host(url: &Url) -> bool {
    url.port().is_none()
        && matches!(
            url.host_str(),
            Some("html5.gamemonetize.co" | "deadlydescent.net" | "blockblastmaster.io")
        )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let home = Url::parse(HOME)?;
            let origin = home.origin();
            let mut launch_url = home.clone();
            launch_url.query_pairs_mut().extend_pairs([
                ("utm_source", "github"),
                ("utm_medium", "referral"),
                ("utm_campaign", CAMPAIGN),
            ]);

            let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(launch_url))
                .title("Browser Game Hub")
                .inner_size(1280.0, 850.0)
                .min_inner_size(460.0, 420.0)
                .on_navigation(move |url| {
                    if url.scheme() != "https" {
                        return false;
                    }
                    #[cfg(any(target_os = "macos", target_os = "android"))]
                    if is_game_host(url) {
                        return true;
                    }
                    url.origin() == origin
                })
                .on_new_window(|_, _| NewWindowResponse::Deny);

            // These WebViews may apply navigation policy to iframes. Keep the main
            // page on Browser Game Hub while allowing its game hosts in iframes.
            #[cfg(any(target_os = "macos", target_os = "android"))]
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
