use chrono::prelude::*;
use sqlx::mysql::MySqlPoolOptions;
// use sqlx::mysql::MySqlRow;
// use rust_decimal::Decimal;
// use std::str::FromStr;
use sqlx::Row;
use std::env;

#[tokio::main]
async fn main() {
    connect_to_database().await.unwrap();

    let current_datetime = Local::now();
    let formatted_date_time = current_datetime.format("%d-%m-%Y %H:%M").to_string();

    let first_payment: Payment = Payment {
        amount: "810.30".to_string(),
        description: String::from("This was the second payment made"),
        account_name: String::from("Dave"),
        payment_date: formatted_date_time,
    };

    add_to_payment_table(first_payment).await.unwrap();

    let req = get_payment_by_id(1).await.unwrap();

    println!("Payment {:?}", req)
}

#[derive(sqlx::FromRow, Debug)]
struct Payment {
    amount: String,
    description: String,
    account_name: String,
    payment_date: String,
}
fn load_env() {
    dotenv::dotenv().ok();
}
async fn connection() -> sqlx::Pool<sqlx::MySql> {
    load_env();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    MySqlPoolOptions::new().connect(url.as_str()).await.unwrap()
}

async fn connect_to_database() -> std::result::Result<(), sqlx::Error> {
    let pool = connection().await;

    // QueryAs<'_, DB, O, <DB as HasArguments>::Arguments>
    match sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS store_payments (
            id INT AUTO_INCREMENT PRIMARY KEY,
            amount VARCHAR(255) NOT NULL,
            account_name VARCHAR(255) NOT NULL,
            description TEXT,
            payment_date VARCHAR(255) NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    {
        Ok(_) => println!("Created payment table"),
        Err(err) => {
            println!("Error creating payment table: {}", err);
            return Err(err);
        }
    };

    Ok(())
}

async fn add_to_payment_table(data: Payment) -> std::result::Result<(), sqlx::Error> {
    let amount = data.amount;
    let description = data.description;
    let account_name = data.account_name;
    let date_of_payment = data.payment_date;

    let pool = connection().await;

    sqlx::query(
        r#"
            INSERT INTO store_payments (amount, description, account_name, payment_date) VALUES (?, ?, ?, ?)
        "#,
    )
   
    .bind(amount)
    .bind(description)
    .bind(account_name)
    .bind(date_of_payment)
    .execute(&pool)
    .await?;

    Ok(())
}

async fn get_payment_by_id(id: i32) -> Result<Payment, sqlx::Error> {
    let pool = connection().await;
    let row = sqlx::query(
        r#"
                SELECT * FROM store_payments WHERE id = ?
            "#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match &row {
        Some(row) => {
            let payment = Payment {
                amount: row.get::<String, _>("amount"),
                description: row.get::<String, _>("description"),
                account_name: row.get::<String, _>("account_name"),
                payment_date: row.get::<String, _>("payment_date"),
            };
            Ok(payment)
        }

        None => Err(sqlx::Error::RowNotFound),
    }
}
