use std::{fs::read_to_string, path::PathBuf};

use crate::types::DataPage;
use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;

pub(crate) fn is_dev() -> bool {
    std::env::var("MODE").ok().as_deref() == Some("development")
}

pub(crate) fn dist_dir() -> PathBuf {
    PathBuf::from("dist")
}

fn vite_origin() -> String {
    std::env::var("VITE_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string())
}

// IMPORTANT: this must match your Vite dev entry URL.
// If your Vite project root is ../web, and main.ts is under that root at /src/main.ts,
// this is correct. If you use /main.ts or /resources/js/app.ts, change it.
fn vite_entry() -> String {
    std::env::var("VITE_ENTRY").unwrap_or_else(|_| "/src/main.ts".to_string())
}

pub(crate) fn response_with_html<T>(req: &HttpRequest, props: T, component: String) -> HttpResponse
where
    T: Serialize,
{
    let data_page = DataPage::new(component, props, req.uri().to_string());

    let data_page_str = serde_json::to_string(&data_page).unwrap();

    let html = if is_dev() {
        dev_html_shell(&data_page_str)
    } else {
        let html_path = dist_dir().join("index.html");
        let html = read_to_string(&html_path)
            .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", html_path, e));
        html.replace("{{DATA_PAGE}}", &data_page_str)
    };

    // Replace the placeholder with the actual data-page attribute
    let html = html.replace("{{DATA_PAGE}}", &data_page_str);

    // Serve the modified HTML
    HttpResponse::Ok().content_type("text/html").body(html)
}

fn dev_html_shell(data_page_json: &str) -> String {
    let origin = vite_origin();
    let entry = vite_entry();

    // Vite backend integration: include @vite/client and your entry module.
    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Dev</title>
  </head>
  <body>
    <div id="app" data-page='{data_page}'></div>

    <script type="module">
        import RefreshRuntime from '{origin}/@react-refresh'
        RefreshRuntime.injectIntoGlobalHook(window)
        window.$RefreshReg$ = () => {{}}
        window.$RefreshSig$ = () => (type) => type
        window.__vite_plugin_react_preamble_installed__ = true
    </script>

    <script type="module" src="{origin}/@vite/client"></script>
    <script type="module" src="{origin}{entry}"></script>
  </body>
</html>
"#,
        origin = origin,
        entry = entry,
        data_page = data_page_json
    )
}
