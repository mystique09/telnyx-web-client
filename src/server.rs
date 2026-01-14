use std::{fs::read_to_string, path::PathBuf};

use actix_files::Files;
use actix_inertia::{VersionMiddleware, inertia_responder::InertiaResponder};
use actix_web::{
    App, Error, HttpRequest, HttpResponse, Responder,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::{Compress, Logger, NormalizePath},
    web,
};
use serde::Serialize;

use crate::types::{DataPage, Empty};

fn is_dev() -> bool {
    std::env::var("MODE").ok().as_deref() == Some("development")
}

/** The dist/ directory must be relative to the final binary
Example:
```
output/
    dist/
    server
    .env
```
*/
fn dist_dir() -> PathBuf {
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

pub fn create_web_service() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let dist = dist_dir();

    let mut app = App::new()
        .wrap(NormalizePath::trim())
        .wrap(Compress::default())
        .wrap(Logger::default())
        .route("/", web::get().to(index))
        .service(
            web::scope("/version")
                .wrap(VersionMiddleware::new("1".to_string()))
                .route("", web::get().to(version)),
        );

    // Production: serve built assets from dist/
    if !is_dev() {
        app = app
            .service(Files::new("/assets", dist.join("assets")).prefer_utf8(true))
            .service(Files::new("/", dist).prefer_utf8(true));
    }
    app
}

#[derive(Serialize)]
struct HomepageProps {
    message: String,
}

async fn index(req: HttpRequest) -> impl Responder {
    let props = HomepageProps {
        message: "Hello, from Telnyx-Web-Client".to_string(),
    };

    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("App", props).respond_to(&req)
    } else {
        response_with_html(&req, props, "App".to_string())
    }
}

async fn version(req: HttpRequest) -> impl Responder {
    if req.headers().contains_key("x-inertia") {
        InertiaResponder::new("VersionPage", Empty).respond_to(&req)
    } else {
        response_with_html(&req, Empty, "VersionPage".to_string())
    }
}

fn response_with_html<T>(req: &HttpRequest, props: T, component: String) -> HttpResponse
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
