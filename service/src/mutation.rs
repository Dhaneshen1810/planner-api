use crate::types::UpdateTaskRequest;
use ::entity::task::{self, Entity as Task, Model, RecurringOption};
use chrono::{Datelike, Local, Weekday};
use prelude::Date;
use sea_orm::{prelude::Expr, *};
pub struct Mutation;

impl Mutation {
    pub async fn add_task(db: &DbConn, new_task: task::Model) -> Result<task::Model, DbErr> {
        let result = task::ActiveModel {
            title: Set(new_task.title.to_owned()),
            date: Set(new_task.date),
            time: Set(new_task.time),
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
        date: Option<Date>,
        time: Option<String>,
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
            time: Set(time),
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
            active_task.time = Set(update.time);
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

    pub async fn reset_due_tasks(db: &DbConn) -> Result<u64, DbErr> {
        let today = Local::now().date_naive(); // Get today's date (YYYY-MM-DD)
        let weekday = match today.weekday() {
            Weekday::Mon => "MONDAY",
            Weekday::Tue => "TUESDAY",
            Weekday::Wed => "WEDNESDAY",
            Weekday::Thu => "THURSDAY",
            Weekday::Fri => "FRIDAY",
            Weekday::Sat => "SATURDAY",
            Weekday::Sun => "SUNDAY",
        };

        let query = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT id, title, date, time, recurring_option::text[] as recurring_option, is_completed, position
            FROM tasks 
            WHERE date = $1::date
              OR array_length(recurring_option, 1) > 0
            "#,
            vec![Value::from(today.to_string())], // Only need today's date
        );

        let all_recurring_tasks_and_for_due_today: Vec<Model> =
            task::Entity::find().from_raw_sql(query).all(db).await?;

        let filtered_tasks: Vec<Model> = all_recurring_tasks_and_for_due_today
            .into_iter()
            .filter(|task| {
                task.date == Some(today)
                    || task.recurring_option.iter().any(|opt| match opt {
                        RecurringOption::Monday => weekday == "MONDAY",
                        RecurringOption::Tuesday => weekday == "TUESDAY",
                        RecurringOption::Wednesday => weekday == "WEDNESDAY",
                        RecurringOption::Thursday => weekday == "THURSDAY",
                        RecurringOption::Friday => weekday == "FRIDAY",
                        RecurringOption::Saturday => weekday == "SATURDAY",
                        RecurringOption::Sunday => weekday == "SUNDAY",
                    })
            })
            .collect();

        // Fetch task IDs that should be updated
        let task_ids: Vec<i32> = filtered_tasks.iter().map(|task| task.id).collect();

        // TODO complete the rest of the code

        if task_ids.is_empty() {
            println!("No tasks to update.");
            return Ok(0);
        }

        // Perform bulk update to set is_completed = false
        let result = Task::update_many()
            .col_expr(task::Column::IsCompleted, Expr::value(false))
            .filter(task::Column::Id.is_in(task_ids))
            .exec(db)
            .await?;

        println!("Updated {} tasks due today", result.rows_affected);
        Ok(result.rows_affected)
    }

    // pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     Post::delete_many().exec(db).await
    // }
}
