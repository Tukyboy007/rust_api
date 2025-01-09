use crate::models::models::{CreatedOrg, DailyStats};
use anyhow::Result;
use chrono::NaiveDate;
use log::info;
use sqlx::{Pool, Postgres}; // If using anyhow

pub async fn check_db(pool: &Pool<Postgres>, db_type: &str) -> Result<()> {
    let row: (String, i32) = sqlx::query_as("SELECT version(), pg_backend_pid()")
        .fetch_one(pool)
        .await?;
    info!("🔌 {} Connection Details:", db_type);
    info!("└─ PostgreSQL Version: {}", row.0);
    info!("└─ Backend PID: {}", row.1);
    info!("└─ Pool Size: {}", pool.size());
    Ok(())
}

pub async fn day_mng_count(pool: &Pool<Postgres>) -> Result<i64> {
    let (count,) = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) 
        FROM bbd_user_logs 
        WHERE created_date BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '1 DAY' - INTERVAL '1 SECOND'
        AND country_code = 'mng'"
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_daily_stats(pool: &Pool<Postgres>) -> Result<Vec<DailyStats>> {
    let rows = sqlx::query_as::<_, (NaiveDate, i64)>(
        "SELECT DATE(created_date) as day, COUNT(*) as user_count 
         FROM bbd_user_logs 
         WHERE created_date BETWEEN CURRENT_DATE - INTERVAL '6 day' AND CURRENT_DATE
         AND pass_type = 'i'
         GROUP BY day 
         ORDER BY day",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(day, user_count)| DailyStats { day, user_count })
        .collect())
}

pub async fn weeknd_mng_total(pool: &Pool<Postgres>) -> Result<i64> {
    let (count,) = sqlx::query_as::<_, (i64,)>(
        "select
              	COUNT(*) as total_user_count
              from
              	bbd_user_logs
              where
              	created_date between current_date - interval '6 day' and current_date - interval '1 second'
              	-- Past 7 days
              and country_code = 'mng'",
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn yesterday_count(pool: &Pool<Postgres>) -> Result<Vec<CreatedOrg>> {
    let rows = sqlx::query_as::<_, (String, i64)>(
        "select
        	    country_code,
        	    COUNT(*)
            from
        	    bbd_user_logs
            where
        	    created_date between current_date - interval '1 day' and current_date - interval '1 second'
            group by country_code",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(created_org, user_count)| CreatedOrg {
            created_org,
            user_count,
        })
        .collect())
}
