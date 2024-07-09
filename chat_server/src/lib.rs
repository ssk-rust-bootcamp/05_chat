mod config;
mod handlers;
use std::{ops::Deref, sync::Arc};

use axum::{
    routing::{get, patch, post},
    Router,
};
pub use config::AppConfig;
use handlers::{
    create_chat_handler, delete_chat_handler, index_handler, list_chat_handler, list_message_handler,
    send_message_handler, signin_handler, signup_handler, update_chat_handler,
};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
}
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
    }
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);
    let api = Router::new()
        .route("/singin", post(signin_handler))
        .route("/singup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chat/:id/message", get(list_message_handler));

    Router::new().route("/", get(index_handler)).nest("/api", api).with_state(state)
}
