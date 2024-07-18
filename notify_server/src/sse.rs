use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::{sse::Event, Sse},
    Extension,
};
use chat_core::User;
use futures::Stream;
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tracing::info;

use crate::AppSate;

const CHANNEL_CAPACITY: usize = 256;
pub(crate) async fn sse_handler(
    Extension(user): Extension<User>,
    State(state): State<AppSate>,
    // TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // info!("`{}` connected to server", user_agent.as_str());
    let user_id = user.id as u64;
    let users = &state.users;
    let rx = if let Some(tx) = users.get(&user_id) {
        tx.subscribe()
    } else {
        let (tx, rx) = broadcast::channel(CHANNEL_CAPACITY);
        state.users.insert(user_id, tx);
        rx
    };
    info!("user `{}` subscribed to broadcast channel", user_id);

    let stream = BroadcastStream::new(rx).filter_map(|v| v.ok()).map(|v| {
        let name = match v.as_ref() {
            crate::notify::AppEvent::NewChat(_) => "NewChat",
            crate::notify::AppEvent::AddToChat(_) => "AddToChat",
            crate::notify::AppEvent::RemoveFromChat(_) => "RemoveFromChat",
            crate::notify::AppEvent::NewMessage(_) => "NewMessage",
        };
        let v = serde_json::to_string(&v).expect("Failed to serialize event");
        Ok(Event::default().data(v).event(name))
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
