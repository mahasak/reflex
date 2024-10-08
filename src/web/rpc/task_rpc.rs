use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::task::{Task, TaskBmc, TaskForCreate, TaskForUpdate};
use crate::web::Result;
use crate::web::rpc::{ParamForCreate, ParamForUpdate, ParamsIded};

pub async fn create_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamForCreate<TaskForCreate>
) -> Result<Task> {
    let ParamForCreate { data } = params;

    let id = TaskBmc::create(&ctx, &mm, data   ).await?;
    let task = TaskBmc::get(&ctx, &mm, id).await?;


    Ok(task)
}

pub async fn list_task(
    ctx: Ctx,
    mm: ModelManager,
) -> Result<Vec<Task>> {
    let tasks = TaskBmc::list(&ctx, &mm).await?;

    Ok(tasks)
}

pub async fn update_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamForUpdate<TaskForUpdate>
) -> Result<Task> {
    let ParamForUpdate { id, data } = params;
    TaskBmc::update(&ctx, &mm, id, data).await?;
    let task = TaskBmc::get(&ctx, &mm, id).await?;

    Ok(task)
}

pub async fn delete_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsIded
) -> Result<Task> {
    let ParamsIded { id } = params;
    let task = TaskBmc::get(&ctx, &mm, id).await?;
    TaskBmc::delete(&ctx, &mm, id).await?;

    Ok(task)
}