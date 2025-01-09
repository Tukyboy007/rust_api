pub mod routes {
    use crate::routes::routes::{
        count_yesterday, health_check, hello, test_db_connection, user_count, weekend_counts,
        weekend_total,
    };
    use actix_web::web;

    pub fn configure_health(cfg: &mut web::ServiceConfig) {
        cfg.service(hello)
            .service(health_check)
            .service(test_db_connection);
    }
    pub fn configure_user(cfg: &mut web::ServiceConfig) {
        cfg.service(user_count)
            .service(weekend_counts)
            .service(weekend_total)
            .service(count_yesterday);
    }
}
