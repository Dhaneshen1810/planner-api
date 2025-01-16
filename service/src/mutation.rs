use ::entity::{task, task::Entity as Task};
use prelude::Date;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn add_task(db: &DbConn, new_task: task::Model) -> Result<task::Model, DbErr> {
        let result = task::ActiveModel {
            title: Set(new_task.title.to_owned()),
            date: Set(new_task.date),
            recurring_option: Set(new_task.recurring_option.clone()),
            is_completed: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(result)
    }

    pub async fn update_task_by_id(
        db: &DbConn,
        id: i32,
        title: String,
        date: Date,
        recurring_option: Option<task::RecurringOption>,
        is_completed: bool,
    ) -> Result<task::Model, DbErr> {
        let task: task::ActiveModel = Task::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find task.".to_owned()))
            .map(Into::into)?;

        task::ActiveModel {
            id: task.id,
            title: Set(title),
            date: Set(date),
            recurring_option: Set(recurring_option),
            is_completed: Set(is_completed),
        }
        .update(db)
        .await
    }

    pub async fn delete_task_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let task: task::ActiveModel = Task::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find task.".to_owned()))
            .map(Into::into)?;

        task.delete(db).await
    }

    // pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     Post::delete_many().exec(db).await
    // }
}
