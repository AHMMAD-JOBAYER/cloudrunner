use crate::models::{NixOsConfig, Teacher};
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

// Teacher database operations
pub async fn create_teacher(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    full_name: &str,
    department: Option<&str>,
) -> Result<Teacher, sqlx::Error> {
    sqlx::query_as::<_, Teacher>(
        r#"
        INSERT INTO teachers (email, password_hash, full_name, department)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(full_name)
    .bind(department)
    .fetch_one(pool)
    .await
}

pub async fn get_teacher_by_email(pool: &PgPool, email: &str) -> Result<Teacher, sqlx::Error> {
    sqlx::query_as::<_, Teacher>("SELECT * FROM teachers WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await
}

pub async fn get_teacher_by_id(pool: &PgPool, id: Uuid) -> Result<Teacher, sqlx::Error> {
    sqlx::query_as::<_, Teacher>("SELECT * FROM teachers WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update_reset_token(
    pool: &PgPool,
    teacher_id: Uuid,
    reset_token: &str,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE teachers 
        SET reset_token = $1, reset_token_expires_at = $2
        WHERE id = $3
        "#,
    )
    .bind(reset_token)
    .bind(expires_at)
    .bind(teacher_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_teacher_by_reset_token(
    pool: &PgPool,
    token: &str,
) -> Result<Teacher, sqlx::Error> {
    sqlx::query_as::<_, Teacher>(
        "SELECT * FROM teachers WHERE reset_token = $1 AND reset_token_expires_at > NOW()",
    )
    .bind(token)
    .fetch_one(pool)
    .await
}

pub async fn update_password(
    pool: &PgPool,
    teacher_id: Uuid,
    new_password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE teachers 
        SET password_hash = $1, reset_token = NULL, reset_token_expires_at = NULL
        WHERE id = $2
        "#,
    )
    .bind(new_password_hash)
    .bind(teacher_id)
    .execute(pool)
    .await?;
    Ok(())
}

// NixOS Config database operations
pub async fn create_nixos_config(
    pool: &PgPool,
    teacher_id: Uuid,
    filename: &str,
    content: &str,
) -> Result<NixOsConfig, sqlx::Error> {
    let file_size = content.len() as i32;

    sqlx::query_as::<_, NixOsConfig>(
        r#"
        INSERT INTO nixos_configs (teacher_id, filename, content, file_size)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(teacher_id)
    .bind(filename)
    .bind(content)
    .bind(file_size)
    .fetch_one(pool)
    .await
}

pub async fn get_teacher_configs(
    pool: &PgPool,
    teacher_id: Uuid,
) -> Result<Vec<NixOsConfig>, sqlx::Error> {
    sqlx::query_as::<_, NixOsConfig>(
        "SELECT * FROM nixos_configs WHERE teacher_id = $1 ORDER BY created_at DESC",
    )
    .bind(teacher_id)
    .fetch_all(pool)
    .await
}

pub async fn get_config_by_id(
    pool: &PgPool,
    config_id: Uuid,
    teacher_id: Uuid,
) -> Result<NixOsConfig, sqlx::Error> {
    sqlx::query_as::<_, NixOsConfig>(
        "SELECT * FROM nixos_configs WHERE id = $1 AND teacher_id = $2",
    )
    .bind(config_id)
    .bind(teacher_id)
    .fetch_one(pool)
    .await
}

pub async fn update_nixos_config(
    pool: &PgPool,
    config_id: Uuid,
    teacher_id: Uuid,
    content: &str,
) -> Result<NixOsConfig, sqlx::Error> {
    let file_size = content.len() as i32;

    sqlx::query_as::<_, NixOsConfig>(
        r#"
        UPDATE nixos_configs 
        SET content = $1, file_size = $2
        WHERE id = $3 AND teacher_id = $4
        RETURNING *
        "#,
    )
    .bind(content)
    .bind(file_size)
    .bind(config_id)
    .bind(teacher_id)
    .fetch_one(pool)
    .await
}

pub async fn delete_nixos_config(
    pool: &PgPool,
    config_id: Uuid,
    teacher_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM nixos_configs WHERE id = $1 AND teacher_id = $2")
        .bind(config_id)
        .bind(teacher_id)
        .execute(pool)
        .await?;
    Ok(())
}
