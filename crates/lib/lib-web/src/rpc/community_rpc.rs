use lib_core::{
    ctx::Ctx,
    model::{
        ModelManager,
        community::{Community, CommunityBmc},
    },
};

use crate::error::Result;

use super::ParamsById;

pub async fn get_community(ctx: Ctx, mm: ModelManager, params: ParamsById) -> Result<Community> {
    let ParamsById { id } = params;

    let community = CommunityBmc::get(&ctx, &mm, id).await?;

    Ok(community)
}
