use crate::model::{base, ModelManager};
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;
use crate::ctx::Ctx;
use crate::model::base::DbBmc;

#[derive(Debug, Clone, Fields, Serialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Clone, Fields, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Debug, Clone, Fields, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

// region: Backend Model Controller
pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task: TaskForCreate,
    ) -> Result<i64> {
        base::create::<Self, _>(_ctx, mm, task).await
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(_ctx, mm, id).await
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(_ctx, mm, id).await
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        base::list::<Self, _>(_ctx, mm).await
    }

    pub async fn update(_ctx: &Ctx, mm: &ModelManager, id: i64, task: TaskForUpdate) -> Result<()> {
        base::update::<Self, _>(_ctx, mm, id, task).await
    }


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
        assert_eq!(tasks[0].title.to_string(), "test_1");
        assert_eq!(tasks[1].title.to_string(), "test_2");

        Ok(())
    }
    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        dotenv().ok();
        // -- Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - task 01";
        let fx_title_new = "test_update_ok - task 01 new";
        let fx_task = _dev_utils::seed_tasks(&ctx, &mm,&[fx_title]).await.unwrap().remove(0);

        // -- Exec

        TaskBmc::update(&ctx, &mm, fx_task.id,TaskForUpdate { title: Some(fx_title_new.to_string())  } ).await.unwrap();

        // -- Check
        let res = TaskBmc::get(&ctx, &mm, fx_task.id).await.unwrap();
        assert_eq!(res.title, fx_title_new.to_string());

        Ok(())
    }
}
// endregion: Unit Test