use super::base::DbBmc;
use super::error::{Error, Result};
use super::{ModelManager, base};
use crate::ctx::Ctx;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use uuid::Uuid;

// region:    --- User Types

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

struct UserForInsert {
    username: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    // -- pwd/token
    pub pwd: Option<String>, // encrypted #_scheme_id_#...
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,
    // -- token info
    pub token_salt: Uuid,
}

/// Marker trait
pub trait UserBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

// endregion: --- User Types

struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "app_user";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_first_by_username(
        ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<User>> {
        let db = mm.db();
        let sql = format!("SELECT * from {} WHERE username LIKE $1", Self::TABLE);
        let user = sqlx::query_as::<_, User>(&sql)
            .bind(username)
            .fetch_one(db)
            .await?;

        Ok(Some(user))
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    use crate::_dev_util;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_first_ok_demo1() -> Result<()> {
        // S
        let mm = &_dev_util::init_test().await;
        let ctx = &Ctx::root_ctx();
        let db = mm.db();
        let fx_username = "demo1";

        // E
        let res: User = UserBmc::get_first_by_username(ctx, mm, fx_username)
            .await?
            .context("Should have user demo1")?;

        // A
        assert_eq!(fx_username, res.username);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_first_ok_demo2() -> Result<()> {
        // S
        let mm = &_dev_util::init_test().await;
        let ctx = &Ctx::root_ctx();
        let db = mm.db();
        let fx_id = 1000;
        let fx_username = "demo1";

        // E
        let res: User = UserBmc::get::<User>(ctx, mm, fx_id).await?;

        // A
        assert_eq!(fx_username, res.username);
        Ok(())
    }
}

// endregion: --- Tests
