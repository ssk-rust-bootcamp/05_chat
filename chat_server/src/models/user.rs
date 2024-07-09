use std::mem;

use argon2::PasswordHasher;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use sqlx::PgPool;

use crate::error::AppError;

use super::User;

#[allow(dead_code)]
impl User {
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as("SELECT * FROM users WHERE email=$1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn create(email: &str, fullname: &str, password: &str, pool: &PgPool) -> Result<Self, AppError> {
        let password_hash = hash_password(password)?;
        let user = sqlx::query_as("INSERT INTO users (email, fullname, password_hash) VALUES ($1, $2, $3) RETURNING *")
            .bind(email)
            .bind(fullname)
            .bind(password_hash)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }
    pub async fn verify(email: &str, password: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email=$1")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid = verify_password(password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}
fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}
fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;
    //verify password
    let is_valid = argon2.verify_password(password.as_bytes(), &password_hash).is_ok();
    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use anyhow::Result;
    use sqlx_db_tester::TestPg;

    #[test]
    fn hash_password_and_verify_should_work() -> Result<()> {
        let password = "password123";
        let password_hash = hash_password(password)?;
        assert_eq!(password_hash.len(), 97);
        assert!(verify_password(password, &password_hash)?);
        Ok(())
    }

    #[tokio::test]
    async fn create_and_verify_user_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://root:root@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let email = "test@test.com";
        let name = "test user";
        let password = "password123";
        let user = User::create(email, name, password, &pool).await?;
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, name);
        assert!(user.id > 0);
        let found_user = User::find_by_email(email, &pool).await?;
        let user = found_user.unwrap();
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, name);

        let is_valid = User::verify(email, password, &pool).await?;
        assert_eq!(is_valid.unwrap().email, email);
        Ok(())
    }
}
