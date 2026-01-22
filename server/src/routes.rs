use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Router;

use crate::handlers::auth_handler::AuthHandler;
use crate::handlers::chapter_handler::ChapterHandler;
use crate::handlers::export_handler::ExportHandler;
use crate::handlers::import_handler::ImportHandler;
use crate::handlers::tag_handler::TagHandler;
use crate::handlers::word_handler::WordHandler;
use crate::handlers::wordbook_handler::WordbookHandler;
use crate::state::AppState;

pub struct AppRouter;

impl AppRouter {
    pub fn create(state: AppState) -> Router {
        let auth_routes = Router::new()
            .route("/register", post(AuthHandler::register))
            .route("/login", post(AuthHandler::login))
            .route("/logout", post(AuthHandler::logout))
            .route("/me", get(AuthHandler::me));

        let tag_routes = Router::new()
            .route("/", get(TagHandler::list).post(TagHandler::create))
            .route("/{id}", put(TagHandler::update).delete(TagHandler::delete));

        let wordbook_routes = Router::new()
            .route("/", get(WordbookHandler::list).post(WordbookHandler::create))
            .route(
                "/{id}",
                get(WordbookHandler::get)
                    .put(WordbookHandler::update)
                    .delete(WordbookHandler::delete),
            )
            .route("/{wordbook_id}/chapters", get(ChapterHandler::list).post(ChapterHandler::create))
            .route(
                "/{wordbook_id}/chapters/{chapter_id}",
                put(ChapterHandler::update).delete(ChapterHandler::delete),
            );

        let word_routes = Router::new()
            .route("/chapters/{chapter_id}/words", get(WordHandler::list).post(WordHandler::create))
            .route("/chapters/{chapter_id}/words/batch", delete(WordHandler::batch_delete))
            .route("/chapters/{chapter_id}/words/batch/tags", post(WordHandler::batch_update_tags))
            .route(
                "/chapters/{chapter_id}/words/{word_id}",
                put(WordHandler::update).delete(WordHandler::delete),
            )
            .route(
                "/chapters/{chapter_id}/words/{word_id}/tags",
                put(WordHandler::update_tags),
            );

        let import_routes = Router::new()
            .route("/templates", get(ImportHandler::download_template))
            .route("/wordbooks", post(ImportHandler::import_wordbook))
            .route(
                "/wordbooks/{wordbook_id}/chapters",
                post(ImportHandler::import_chapter),
            );

        let export_routes = Router::new()
            .route("/wordbooks/{wordbook_id}", get(ExportHandler::export_wordbook))
            .route(
                "/wordbooks/{wordbook_id}/chapters/{chapter_id}",
                get(ExportHandler::export_chapter),
            );

        Router::new()
            .nest("/api/auth", auth_routes)
            .nest("/api/tags", tag_routes)
            .nest("/api/wordbooks", wordbook_routes)
            .nest("/api", word_routes)
            .nest("/api/import", import_routes)
            .nest("/api/export", export_routes)
            .with_state(state)
    }
}