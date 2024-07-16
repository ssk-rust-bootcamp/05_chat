use super::{ChatUser, Workspace};
use crate::{error::AppError, AppState};
#[allow(unused)]
impl AppState {
    pub async fn create_workspace(&self, name: &str, user_id: u64) -> Result<Workspace, AppError> {
        let ws = sqlx::query_as(r#"INSERT INTO workspaces (name, owner_id) VALUES ($1, $2) RETURNING *"#)
            .bind(name)
            .bind(user_id as i64)
            .fetch_one(&self.pool)
            .await?;

        Ok(ws)
    }

    pub async fn update_workspace_owner(&self, id: u64, owner_id: u64) -> Result<Workspace, AppError> {
        // update owner_id in two cases 1) owner_id = 0 2) owner's ws_id = id
        let ws = sqlx::query_as(r#"UPDATE workspaces SET owner_id = $1 WHERE id = $2 RETURNING *"#)
            .bind(owner_id as i64)
            .bind(id as i64)
            .fetch_one(&self.pool)
            .await?;

        Ok(ws)
    }

    pub async fn find_workspace_by_name(&self, name: &str) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(r#"SELECT * FROM workspaces WHERE name = $1"#)
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;
        Ok(ws)
    }

    pub async fn find_workspace_by_id(&self, id: u64) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(r#"SELECT * FROM workspaces WHERE id = $1"#)
            .bind(id as i64)
            .fetch_optional(&self.pool)
            .await?;
        Ok(ws)
    }

    pub async fn fetch_workspace_all_chat_users(&self, id: u64) -> Result<Vec<ChatUser>, AppError> {
        let users = sqlx::query_as(
            r#"
        SELECT id, fullname, email
        FROM users
        WHERE ws_id = $1 order by id

            "#,
        )
        .bind(id as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Result;

    use super::*;
    use crate::models::CreateUser;
    #[tokio::test]
    async fn workspace_should_create_and_set_owner() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.create_workspace("test", 0).await?;
        let input = CreateUser::new(&ws.name, "test", "123456@qq.com", "password1233");
        let user = state.create_user(&input).await?;
        let ws = state
            .update_workspace_owner(ws.id as _, user.id as _)
            .await?;
        assert_eq!(user.ws_id, ws.id);
        assert_eq!(ws.name, "test");

        let ws = state
            .update_workspace_owner(ws.id as _, user.id as _)
            .await?;
        assert_eq!(ws.owner_id, user.id);

        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_find_by_name() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let _ws = state.create_workspace("test", 0).await?;
        let ws = state.find_workspace_by_name("test").await?;
        assert_eq!(ws.unwrap().name, "test");
        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_fetch_all_chat_users() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.create_workspace("test", 0).await?;
        let input = CreateUser::new(&ws.name, "test1", "123456@qq.com", "password1233");
        let user1 = state.create_user(&input).await?;
        let input = CreateUser::new(&ws.name, "test2", "1234567@qq.com", "password1233");
        let user2 = state.create_user(&input).await?;
        let users = state.fetch_workspace_all_chat_users(ws.id as _).await?;

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].id, user1.id);
        assert_eq!(users[1].id, user2.id);

        Ok(())
    }
}
