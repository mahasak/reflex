use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::{merchant_channel, ModelManager};
use crate::model::Result;
use modql::field::{Fields, HasFields};
use modql::filter::{
    FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString,
};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use crate::model::merchant_channel::{MerchantChannel, MerchantChannelBmc, MerchantChannelBy, MerchantChannelForCreate, MerchantChannelForUpdate, MerchantChannelWithoutToken};
use crate::model::task::{Task, TaskBmc, TaskFilter, TaskForCreate, TaskForUpdate};


// region:    --- merchant config Types
#[derive(Debug, Clone, FromRow, Fields, Serialize, Deserialize)]
pub struct MerchantConfig {
    pub channel_id: i64,
    pub app_id: i64,
    pub topic: String,
    pub enabled: bool,
    pub token: String,
}

#[derive(Debug, Clone,FromRow,Fields,Serialize, Deserialize)]
pub struct MerchantConfigWithoutToken {
    pub channel_id: i64,
    pub app_id: i64,
    pub topic: String,
    pub enabled: bool,
}

#[derive(Debug, Clone,FromRow,Fields,Serialize, Deserialize)]
pub struct MerchantConfigForCreate {
    pub app_id: i64,
    pub topic: String,
    pub enabled: bool,
}

#[derive(Debug, Clone,FromRow,Fields,Serialize, Deserialize)]
pub struct MerchantConfigForUpdate {
    pub channel_id: i64,
    pub app_id: i64,
    pub topic: String,
    pub enabled: bool,
}


// endregion:    --- merchant config Types

pub trait MerchantConfigBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
impl MerchantConfigBy for MerchantConfig {}
impl MerchantConfigBy for MerchantConfigWithoutToken {}
impl MerchantConfigBy for MerchantConfigForCreate {}

#[derive(Iden)]
enum MerchantConfigIden {
    ChannelId,
    AppId,
    Topic,
}

pub struct MerchantConfigBmc;

impl DbBmc for MerchantConfigBmc {
    const TABLE: &'static str = "merchant_config";
}


impl MerchantConfigBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        merchant: MerchantConfigForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, merchant).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<MerchantConfigWithoutToken> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_with_token(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<MerchantConfig> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_by_channel_id<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        channel_id: &i64,
    ) -> Result<Option<E>>
    where
        E: MerchantConfigBy,
    {
        let db = mm.db();

        // -- Build query
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::field_idens())
            .and_where(Expr::col(MerchantConfigIden::ChannelId).eq(*channel_id));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let merchant_config = sqlx::query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(merchant_config)
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TaskFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<MerchantConfigWithoutToken>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        merchant: MerchantConfigForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, merchant).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}