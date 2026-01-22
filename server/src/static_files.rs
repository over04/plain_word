use axum::body::Body;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use rust_embed::Embed;

const INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "static"]
struct Assets;

pub struct StaticFiles;

impl StaticFiles {
    pub async fn serve(uri: axum::http::Uri) -> impl IntoResponse {
        let path = uri.path().trim_start_matches('/');
        Self::serve_path(if path.is_empty() { INDEX_HTML } else { path })
    }

    fn serve_path(path: &str) -> Response {
        match Assets::get(path) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(Body::from(content.data.to_vec()))
                    .unwrap()
            }
            None => Self::serve_path(INDEX_HTML),
        }
    }
}