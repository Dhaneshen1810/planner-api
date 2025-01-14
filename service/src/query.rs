use ::entity::{task, task::Entity as Task};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_all_tasks(conn: &DbConn) -> Result<Vec<task::Model>, DbErr> {
        Task::find().all(conn).await
    }

    pub async fn find_task_by_id(db: &DbConn, id: i32) -> Result<Option<task::Model>, DbErr> {
        Task::find_by_id(id).one(db).await
    }
}
