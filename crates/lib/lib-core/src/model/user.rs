use super::base::DbBmc;
use super::error::{Error, Result};
use super::{ModelManager, base};
use crate::crypt::EncryptContent;
use crate::crypt::password::encrypt_pwd;
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

pub struct UserBmc;

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

    pub async fn get_first_by_username<E>(
        ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();
        let sql = format!("SELECT * from {} WHERE username LIKE $1", Self::TABLE);
        let user = sqlx::query_as::<_, E>(&sql)
            .bind(username)
            .fetch_one(db)
            .await?;

        Ok(Some(user))
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, clear_pw: String, id: i64) -> Result<()> {
        let db = mm.db();
        let user: UserForLogin = UserBmc::get(ctx, mm, id).await?;
        let enc_pwd = encrypt_pwd(&EncryptContent {
            content: clear_pw,
            salt: user.pwd_salt.to_string(),
        })?;

        let _res = sqlx::query("UPDATE app_user SET pwd = $1 WHERE id = $2 RETURNING *")
            .bind(enc_pwd)
            .bind(id)
            .fetch_one(db)
            .await?;

        Ok(())
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    use crate::{_dev_util, config::core_config, crypt::encrypt_into_b64u};
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
    async fn test_second_ok_demo1() -> Result<()> {
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

    #[tokio::test]
    #[serial]
    async fn test_update_pw_ok_demo1() -> Result<()> {
        // S
        let mm = &_dev_util::init_test().await;
        let ctx = &Ctx::root_ctx();
        let db = mm.db();
        let fx_id = 1000i64;
        let original_user: UserForLogin = UserBmc::get::<UserForLogin>(ctx, mm, fx_id).await?;

        let fx_clear_pw: String = "my-secure-pw".to_string();
        let fx_enc_content = &EncryptContent {
            content: fx_clear_pw.clone(),
            salt: original_user.pwd_salt.into(),
        };
        let fx_pw_updated = format!(
            "#01#{}",
            encrypt_into_b64u(&core_config().PWD_KEY, fx_enc_content)
                .context("Didn't update pw")
                .unwrap()
        );

        // E
        let res: () = UserBmc::update_pwd(ctx, mm, fx_clear_pw, fx_id).await?;

        // A
        // Assert not an error so update complete
        assert_eq!(res, ());

        let res_get_2: UserForLogin = UserBmc::get::<UserForLogin>(ctx, mm, fx_id).await?;
        if let Some(updated_pw) = res_get_2.pwd {
            assert_eq!(updated_pw, fx_pw_updated);
        }

        Ok(())
    }
}

// endregion: --- Tests
