use axum::{
    extract::{FromRequestParts, Path, Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{error::AppError, AppState, User};

pub async fn verify_chat(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let (mut parts, body) = req.into_parts();
    let Path(chat_id) = Path::<u64>::from_request_parts(&mut parts, &state)
        .await
        .unwrap();
    let user = parts.extensions.get::<User>().unwrap();
    eprintln!("chat_id: {}", chat_id);
    eprintln!("user: {:?}", user);
    if !state
        .is_chat_member(chat_id, user.id as _)
        .await
        .unwrap_or_default()
    {
        let err = AppError::CreateMessageError(format!("User {} are not a member of chat {chat_id}", user.id));
        return err.into_response();
    }

    let req = Request::from_parts(parts, body);
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{body::Body, http::StatusCode, middleware::from_fn_with_state, routing::get, Router};
    use tower::ServiceExt;

    use super::*;
    use crate::middlewares::verify_token;

    async fn handler(_req: Request) -> impl IntoResponse {
        (StatusCode::OK, "ok")
    }

    #[tokio::test]
    async fn verify_chat_middleware_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let user = state.find_user_by_id(1).await?.expect("user should exist");
        eprintln!("user: {:?}", user);
        let tonken = state.ek.sign(user)?;
        eprintln!("token: {}", tonken);
        let app = Router::new()
            .route("/chat/:id/message", get(handler))
            .layer(from_fn_with_state(state.clone(), verify_chat))
            .layer(from_fn_with_state(state.clone(), verify_token))
            .with_state(state);

        // user in chat
        let req = Request::builder()
            .uri("/chat/1/message")
            .header("Authorization", format!("Bearer {}", tonken))
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        // user not in chat
        let req = Request::builder()
            .uri("/chat/5/message")
            .header("Authorization", format!("Bearer {}", tonken))
            .body(Body::empty())?;

        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }
}
