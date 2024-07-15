use axum::{
    extract::{Multipart, Path, State},
    http::HeaderMap,
    response::IntoResponse,
    Extension, Json,
};
use tokio::fs;
use tracing::{info, warn};

use crate::{error::AppError, AppState, ChatFile, User};

pub(crate) async fn send_message_handler() -> impl IntoResponse {
    "send message"
}

pub(crate) async fn list_message_handler() -> impl IntoResponse {
    "list message"
}

pub(crate) async fn file_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path((ws_id, path)): Path<(i64, String)>,
) -> Result<impl IntoResponse, AppError> {
    if user.ws_id != ws_id {
        return Err(AppError::NotFound(
            "File does not exist or you don't have permission".to_string(),
        ));
    }
    let base_dir = state.config.server.base_dir.join(ws_id.to_string());
    let path = base_dir.join(path);
    info!("Serving file: {}", path.display());
    if !path.exists() {
        return Err(AppError::NotFound("File does not exist".to_string()));
    }
    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    //TODO: streaming
    let body = fs::read(path).await?;
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", mime.to_string().parse().unwrap());

    Ok((headers, body))
}

pub(crate) async fn upload_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let ws_id = user.ws_id as u64;
    let base_dir = state.config.server.base_dir.join(ws_id.to_string());
    let mut files = vec![];
    while let Some(filed) = multipart.next_field().await.unwrap() {
        let filename = filed.file_name().map(|name| name.to_string());
        let (Some(filename), Ok(data)) = (filename, filed.bytes().await) else {
            warn!("Failed to read mutipart filed");
            continue;
        };
        let file = ChatFile::new(&filename, &data);
        let path = file.path(&base_dir);
        if path.exists() {
            info!("File {} already exists {:?}", filename, path);
        } else {
            fs::create_dir_all(path.parent().expect("file path parent should exist")).await?;
            fs::write(path, data).await?;
        }

        files.push(file);
    }
    Ok(Json(files))
}
