use ::entity::task::{self, Entity as Task, Model, RecurringOption};
use chrono::{Datelike, NaiveDate, Weekday};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_all_tasks(conn: &DbConn) -> Result<Vec<task::Model>, DbErr> {
        Task::find().order_by_asc(task::Column::Id).all(conn).await
    }

    pub async fn find_tasks_by_date(
        conn: &DbConn,
        date: NaiveDate,
    ) -> Result<Vec<task::Model>, DbErr> {
        let weekday = match date.weekday() {
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
            vec![Value::from(date.to_string())], // Only need today's date
        );

        let all_recurring_tasks_and_for_due_today: Vec<Model> =
            task::Entity::find().from_raw_sql(query).all(conn).await?;

        let filtered_tasks: Vec<Model> = all_recurring_tasks_and_for_due_today
            .into_iter()
            .filter(|task| {
                task.date == Some(date)
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

        Ok(filtered_tasks)
    }

    pub async fn find_task_by_id(db: &DbConn, id: i32) -> Result<Option<task::Model>, DbErr> {
        Task::find_by_id(id).one(db).await
    }
}
