use ::entity::{task, task::Entity as Task};
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

        println!("Fetching tasks for date: {}", date);
        println!("Corresponding weekday: {}", weekday);

        // let query = Statement::from_sql_and_values(
        //     DbBackend::Postgres,
        //     r#"
        //     SELECT * FROM tasks
        //     WHERE date = $1 OR $2 = ANY(recurring_option)
        //     "#,
        //     [date.into(), weekday.into()],
        // );
        let query = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            SELECT id, title, date, time, recurring_option, is_completed, position
            FROM tasks 
            WHERE (date IS NULL OR date = $1::date) OR $2::recurring_option = ANY(recurring_option)
            "#,
            vec![date.into(), weekday.into()],
        );

        // let tasks: Vec<task::Model> = Task::find().from_raw_sql(query).all(conn).await?;
        let tasks_result = Task::find().from_raw_sql(query).all(conn).await;

        match tasks_result {
            Ok(tasks) => {
                println!("Tasks fetched successfully: {:?}", tasks);
                Ok(tasks)
            }
            Err(err) => {
                println!(
                    "Error fetching tasks for date {} (weekday {}): {:?}",
                    date, weekday, err
                );
                Err(err)
            }
        }
        // Ok(tasks)
    }

    pub async fn find_task_by_id(db: &DbConn, id: i32) -> Result<Option<task::Model>, DbErr> {
        Task::find_by_id(id).one(db).await
    }
}
