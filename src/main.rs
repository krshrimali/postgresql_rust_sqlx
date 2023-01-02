use sqlx::postgres::PgPoolOptions;
// use sqlx::Connection;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://krshrimali:krs@localhost/sample2")
        .await?;

    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool)
    //     .await?;
    // sqlx::query!("CREATE TABLE image_database (idx INT, isCorrect BOOLEAN DEFAULT false)").fetch_all(&pool).await?;
    sqlx::query!("INSERT INTO image_database VALUES (0, true)").fetch_all(&pool).await?;
    sqlx::query!("INSERT INTO image_database VALUES (1, false)").fetch_all(&pool).await?;
    let row = sqlx::query!("select * from image_database")
        .fetch_all(&pool)
        .await?;
    println!("row: {:?}", row);
    // let row2 = sqlx::query!("select bool_val from table_sample")
    //     .fetch_all(&pool)
    //     .await?;
    // println!("row: {:?}", row2);
    // assert_eq!(row.0, 150);

    Ok(())
}
