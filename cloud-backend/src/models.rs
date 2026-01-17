use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Teacher {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: String,
    pub department: Option<String>,
    #[serde(skip_serializing)]
    pub reset_token: Option<String>,
    #[serde(skip_serializing)]
    pub reset_token_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NixOsConfig {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub filename: String,
    pub content: String,
    pub file_size: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub department: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub teacher: TeacherResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub department: Option<String>,
}

impl From<Teacher> for TeacherResponse {
    fn from(teacher: Teacher) -> Self {
        Self {
            id: teacher.id,
            email: teacher.email,
            full_name: teacher.full_name,
            department: teacher.department,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadConfigRequest {
    pub filename: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub id: Uuid,
    pub filename: String,
    pub content: String,
    pub file_size: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NixOsConfig> for ConfigResponse {
    fn from(config: NixOsConfig) -> Self {
        Self {
            id: config.id,
            filename: config.filename,
            content: config.content,
            file_size: config.file_size,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message: message.into(),
            data: None,
        }
    }
}
