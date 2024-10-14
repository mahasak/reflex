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
use crate::model::task::{Task, TaskBmc, TaskFilter, TaskForCreate, TaskForUpdate};
use crate::model::user::{User, UserBy, UserForAuth, UserForLogin};

// region:    --- merchant channel Types
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct MerchantChannel {
    pub id: i64,
    pub ref_id: String,
    pub name: String,
    pub ref_type: String,
    pub token: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct MerchantChannelWithoutToken {
    pub id: i64,
    pub ref_id: String,
    pub name: String,
    pub ref_type: String,
}

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct MerchantChannelForCreate {
    pub ref_id: String,
    pub name: String,
    pub ref_type: String,
    pub token: String,
}

#[derive(Debug, Clone, Fields, Serialize)]
pub struct MerchantChannelForUpdate {
    pub id: i64,
    pub ref_id: String,
    pub name: String,
    pub ref_type: String,
    pub token: String,
}

#[derive(Debug, Clone, Fields, Serialize)]
pub struct MerchantChannelForDelete {
    pub id: i64,
    pub ref_id: String,
    pub name: String,
    pub ref_type: String,
    pub token: String,
}
// endregion:    --- merchant channel Types
pub trait MerchantChannelBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl MerchantChannelBy for MerchantChannel {}
impl MerchantChannelBy for MerchantChannelWithoutToken {}
impl MerchantChannelBy for MerchantChannelForCreate {}
#[derive(Iden)]
enum MerchantChannelIden {
    RefId,
    Name,
    RefType,
}

pub struct MerchantChannelBmc;

impl DbBmc for MerchantChannelBmc {
    const TABLE: &'static str = "merchant_channel";
}

impl MerchantChannelBmc {
    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        merchant: MerchantChannelForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, merchant).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<MerchantChannelWithoutToken> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_with_token(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<MerchantChannel> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn get_by_ref_id<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        ref_id: &str,
    ) -> Result<Option<E>>
    where
        E: MerchantChannelBy,
    {
        let db = mm.db();

        // -- Build query
        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::field_idens())
            .and_where(Expr::col(MerchantChannelIden::RefId).eq(ref_id));

        // -- Exec query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let merchant_channel = sqlx::query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(merchant_channel)
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TaskFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<MerchantChannelWithoutToken>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        merchant: MerchantChannelForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, merchant).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}