use crate::ctx::Ctx;

use super::ModelManager;
use super::error::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::Uuid};

// region:    --- Community Types

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Community {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CommunityForCreate {
    pub name: String,
}

// endregion: --- Community Types

// region:    --- CommunityBmc
pub struct CommunityBmc;

impl CommunityBmc {
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        community_c: CommunityForCreate,
    ) -> Result<i64> {
        let db = mm.db();

        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT INTO community (name) values ($1) returning id")
                .bind(community_c.name)
                .fetch_one(db)
                .await?;

        Ok(id)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Community> {
        let db = mm.db();

        let community = sqlx::query_as::<_, Community>("SELECT * FROM community WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "community",
                id,
            })?;

        Ok(community)
    }
}

// endregion: --- CommunityBmc

// region:    --- Tests
#[cfg(test)]
mod tests {
    use crate::_dev_util;

    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // S
        let mm = _dev_util::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok community name".to_string();

        // E
        let id = CommunityBmc::create(
            &ctx,
            &mm,
            CommunityForCreate {
                name: fx_name.clone(),
            },
        )
        .await
        .unwrap();

        let (_, name) =
            sqlx::query_as::<_, (i64, String)>("SELECT * FROM community WHERE id = $1 LIMIT 1")
                .bind(id)
                .fetch_one(mm.db())
                .await?;

        // A
        assert_eq!(name, fx_name);

        // T
        let count = sqlx::query("DELETE from community WHERE id =$1 returning id")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();

        assert_eq!(count, 1, "Didn't delete a row?");
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_ok() -> Result<()> {
        // S
        let mm = _dev_util::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok community name".to_string();

        // E
        let id = CommunityBmc::create(
            &ctx,
            &mm,
            CommunityForCreate {
                name: fx_name.clone(),
            },
        )
        .await
        .unwrap();

        let Community { id, name } = CommunityBmc::get(&ctx, &mm, id).await.unwrap();

        // A
        assert_eq!(name, fx_name);

        // T
        let count = sqlx::query("DELETE from community WHERE id =$1 returning id")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();

        assert_eq!(count, 1, "Didn't delete a row?");
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // S
        let mm = _dev_util::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_name = "test_create_ok community name".to_string();
        let fx_id = 100;

        // E
        let id = CommunityBmc::create(
            &ctx,
            &mm,
            CommunityForCreate {
                name: fx_name.clone(),
            },
        )
        .await
        .unwrap();

        let result = CommunityBmc::get(&ctx, &mm, fx_id).await;

        // A
        assert!(
            matches!(
                result,
                Err(Error::EntityNotFound {
                    entity: "community",
                    id: fx_id
                }),
            ),
            "EntityNotFound not matching"
        );

        // T
        let count = sqlx::query("DELETE from community WHERE id =$1 returning id")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();

        assert_eq!(count, 1, "Didn't delete a row?");
        Ok(())
    }
}
// endregion: --- Tests
