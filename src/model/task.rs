use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::ctx::Ctx;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

// region: Backend Model Controller
pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task: TaskForCreate,
    ) -> Result<i64> {
        let db = mm.db();
        let (id, ) = sqlx::query_as::<_, (i64,)>("
            INSERT INTO task (title)
            VALUES ($1)
            RETURNING id",
        ).bind(task.title)
            .fetch_one(db)
            .await?;

        Ok(id)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        let db = mm.db();

        let task: Task = sqlx::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "task" ,id })?;

        Ok(task)
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();

        let count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();
        if count == 0 {
            return Err(Error::EntityNotFound { entity: "task" ,id });
        }

        Ok(())
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        let db = mm.db();

        let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM task ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(tasks)
    }

    // pub async fn update(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {}


}
// endregion: Backend Model Controller

// region: Unit Test
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
    use dotenv::dotenv;
    use serial_test::serial;
    use crate::_dev_utils;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        dotenv().ok();
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        // -- Exec
        let task_c = TaskForCreate {
            title: fx_title.to_string()
        };
        let id = TaskBmc::create(&ctx, &mm, task_c).await.unwrap();


        // -- Check
        let task = TaskBmc::get(&ctx, &mm, id).await.unwrap();

        assert_eq!(task.title, fx_title);

        // Clean
        TaskBmc::delete(&ctx, &mm, id).await.unwrap();

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        dotenv().ok();
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        // -- Exec
        let res = TaskBmc::get(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {entity: "task", id: 100})
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        dotenv().ok();
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        // -- Exec
        let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {entity: "task", id: 100})
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        dotenv().ok();
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let tasks = &["test_1", "test_2"];
        _dev_utils::seed_tasks(&ctx, &mm,tasks).await.unwrap();

        // -- Exec
        let tasks = TaskBmc::list(&ctx, &mm).await.unwrap();

        // -- Check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|task| task.title.starts_with("test_"))
            .collect();

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title, "test_1");
        assert_eq!(tasks[1].title, "test_2");

        Ok(())
    }

}
// endregion: Unit Test