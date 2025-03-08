use crate::ctx::Ctx;

use super::ModelManager;
use super::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::Uuid};

// region:    --- Community Types

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Community {
    pub id: i64,
    pub name: String,
    pub members: Vec<i64>,
}

#[derive(Deserialize)]
pub struct CommunityForCreate {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CommunityAddUser {
    pub user_id: i64,
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
}

// endregion: --- CommunityBmc
