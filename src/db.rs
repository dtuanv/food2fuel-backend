use sqlx::PgPool;

pub async fn create_ticket(pool :&PgPool, title: &str) -> Result<(), sqlx::Error>{

}