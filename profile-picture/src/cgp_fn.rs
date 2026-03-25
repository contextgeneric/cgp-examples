#![allow(async_fn_in_trait)]

use aws_sdk_s3::Client;
use cgp::prelude::*;
use image::RgbImage;
use sqlx::PgPool;

pub struct UserId(pub u64);

pub struct User {
    pub name: String,
    pub email: String,
    pub profile_picture_object_id: Option<String>,
}

#[cgp_fn]
pub async fn get_user(
    &self,
    #[implicit] database: &PgPool,
    user_id: &UserId,
) -> anyhow::Result<User> {
    todo!()
}

#[cgp_fn]
pub async fn fetch_storage_object(
    &self,
    #[implicit] storage_client: &Client,
    object_id: &str,
) -> anyhow::Result<Vec<u8>> {
    let output = storage_client
        .get_object()
        .bucket("my-bucket")
        .key(object_id)
        .send()
        .await?;
    let data = output.body.collect().await?.into_bytes().to_vec();
    Ok(data)
}

#[cgp_fn]
#[uses(GetUser, FetchStorageObject)]
pub async fn get_user_profile_picture(&self, user_id: &UserId) -> anyhow::Result<Option<RgbImage>> {
    let user = self.get_user(user_id).await?;

    if let Some(object_id) = user.profile_picture_object_id {
        let data = self.fetch_storage_object(&object_id).await?;
        let image = image::load_from_memory(&data)?.to_rgb8();

        Ok(Some(image))
    } else {
        Ok(None)
    }
}
