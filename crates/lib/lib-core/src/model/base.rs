use sqlx::{postgres::PgRow, FromRow};

use super::{Error, ModelManager, Result};
use crate::ctx::Ctx;

pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
  let db = mm.db();

  let entity = sqlx::query_as::<_, E>(&format!("SELECT * FROM {} WHERE id = $1", MC::TABLE))
      .bind(id)
      .fetch_optional(db)
      .await?
      .ok_or(Error::EntityNotFound {
          entity: MC::TABLE,
          id,
      })?;

  Ok(entity)
}
