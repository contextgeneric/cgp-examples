use aws_sdk_s3::Client;
use cgp::prelude::*;
use image::RgbImage;
use sqlx::{Database, PgPool, Pool};

pub struct UserId(pub u64);

#[derive(sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub email: String,
    pub profile_picture_object_id: Option<String>,
}

#[cgp_fn]
#[async_trait]
pub async fn get_user(
    &self,
    #[implicit] database: &PgPool,
    user_id: &UserId,
) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>(
        "SELECT name, email, profile_picture_object_id FROM users WHERE id = $1",
    )
    .bind(user_id.0 as i64)
    .fetch_one(database)
    .await?;
    Ok(user)
}

#[cgp_fn]
#[async_trait]
#[impl_generics(Db: Database)]
pub async fn generic_get_user(
    &self,
    #[implicit] database: &Pool<Db>,
    user_id: &UserId,
) -> anyhow::Result<User>
where
    i64: sqlx::Type<Db>,
    for<'a> User: sqlx::FromRow<'a, Db::Row>,
    for<'a> i64: sqlx::Encode<'a, Db>,
    for<'a> <Db as sqlx::Database>::Arguments<'a>: sqlx::IntoArguments<'a, Db>,
    for<'a> &'a mut <Db as sqlx::Database>::Connection: sqlx::Executor<'a, Database = Db>,
{
    let user = sqlx::query_as::<_, User>(
        "SELECT name, email, profile_picture_object_id FROM users WHERE id = $1",
    )
    .bind(user_id.0 as i64)
    .fetch_one(database)
    .await?;
    Ok(user)
}

#[cgp_fn]
#[async_trait]
pub async fn fetch_storage_object(
    &self,
    #[implicit] storage_client: &Client,
    #[implicit] profile_pictures_bucket_id: &str,
    object_id: &str,
) -> anyhow::Result<Vec<u8>> {
    let output = storage_client
        .get_object()
        .bucket(profile_pictures_bucket_id)
        .key(object_id)
        .send()
        .await?;
    let data = output.body.collect().await?.into_bytes().to_vec();
    Ok(data)
}

#[cgp_fn]
#[async_trait]
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
