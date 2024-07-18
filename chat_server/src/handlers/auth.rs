use axum::http::StatusCode;
use axum::Json;
use axum::{extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::{AppError, ErrorOutput};
use crate::models::{CreateUser, SigninUser};
use crate::AppState;

#[derive(Debug,ToSchema, Serialize, Deserialize)]
pub struct AuthOutput {
    token: String,
}

#[utoipa::path(
    post,
    path="/api/signup",
    responses((status= 200 ,description="User created",body = AuthOutput),
))]
pub(crate) async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.create_user(&input).await?;
    let token = state.ek.sign(user)?;
    let body = Json(AuthOutput { token });
    Ok((StatusCode::CREATED, body))
}

#[utoipa::path(
    post,
    path = "/api/signin",
    responses(
        (status = 200, description = "User signed in", body = AuthOutput),
    )
)]
pub(crate) async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.verify_user(&input).await?;
    match user {
        Some(user) => {
            let token = state.ek.sign(user)?;
            Ok((StatusCode::OK, Json(AuthOutput { token })).into_response())
        }
        None => {
            let body = Json(ErrorOutput::new("Invalid email or password"));
            Ok((StatusCode::FORBIDDEN, body).into_response())
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use http_body_util::BodyExt;

    use super::*;

    #[tokio::test]
    async fn singup_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateUser::new("none", "ssk", "123456@qq.com", "pass123444");
        let ret = signup_handler(State(state), Json(input))
            .await?
            .into_response();
        assert_eq!(ret.status(), StatusCode::CREATED);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret = serde_json::from_slice::<AuthOutput>(&body)?;
        assert_ne!(ret.token, "");
        Ok(())
    }
    #[tokio::test]
    async fn signup_duplicate_user_should_409() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateUser::new("none", "ssk1", "ssk@acme.org", "pass123444");
        signup_handler(State(state.clone()), Json(input.clone())).await?;
        let ret = signup_handler(State(state), Json(input.clone()))
            .await
            .into_response();
        // dbg!(&ret.status());
        assert_eq!(ret.status(), StatusCode::CONFLICT);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, "email already exists: ssk@acme.org");
        Ok(())
    }

    #[tokio::test]
    async fn signin_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let email = "tchen@acme.org";
        let password = "123456";

        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await?
            .into_response();

        assert_eq!(ret.status(), StatusCode::OK);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret = serde_json::from_slice::<AuthOutput>(&body)?;
        assert_ne!(ret.token, "");

        Ok(())
    }

    #[tokio::test]
    async fn signin_with_non_exist_user_should_403() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let email = "alice@acme.org";
        let password = "Hunter42";
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await
            .into_response();
        assert_eq!(ret.status(), StatusCode::FORBIDDEN);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, "Invalid email or password".to_string());

        Ok(())
    }
}
