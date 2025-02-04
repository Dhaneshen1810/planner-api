use crate::types::UpdateTaskRequest;
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
        recurring_option: Vec<task::RecurringOption>,
        is_completed: bool,
        position: i32,
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
            position: Set(position),
        }
        .update(db)
        .await
    }

    pub async fn update_tasks_bulk(
        db: &DbConn,
        updates: Vec<UpdateTaskRequest>,
    ) -> Result<Vec<task::Model>, DbErr> {
        let mut transaction = db.begin().await?;
        let mut updated_tasks = Vec::new();

        for update in updates {
            let task = task::Entity::find_by_id(update.id)
                .one(&transaction)
                .await?
                .ok_or_else(|| DbErr::Custom(format!("Task with id {} not found", update.id)))?;

            let mut active_task: task::ActiveModel = task.into();
            active_task.title = Set(update.title);
            active_task.date = Set(update.date);
            active_task.recurring_option = Set(update.recurring_option);
            active_task.is_completed = Set(update.is_completed);
            active_task.position = Set(update.position);

            let updated_task = active_task.update(&transaction).await?;
            updated_tasks.push(updated_task);
        }

        transaction.commit().await?;
        Ok(updated_tasks)
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
