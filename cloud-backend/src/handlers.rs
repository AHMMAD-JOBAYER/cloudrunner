use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    auth::{AuthenticatedTeacher, create_jwt},
    db,
    models::*,
};

// Auth handlers
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<TeacherResponse>>, (StatusCode, String)> {
    // Validate email format
    if !payload.email.contains('@') {
        return Err((StatusCode::BAD_REQUEST, "Invalid email format".to_string()));
    }

    // Validate password length
    if payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Check if email already exists
    if db::get_teacher_by_email(&state.db, &payload.email)
        .await
        .is_ok()
    {
        return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
    }

    // Hash password
    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to hash password".to_string(),
        )
    })?;

    // Create teacher
    let teacher = db::create_teacher(
        &state.db,
        &payload.email,
        &password_hash,
        &payload.full_name,
        payload.department.as_deref(),
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok(Json(ApiResponse::success(
        "Registration successful",
        TeacherResponse::from(teacher),
    )))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, (StatusCode, String)> {
    // Get teacher by email
    let teacher = db::get_teacher_by_email(&state.db, &payload.email)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    // Verify password
    let valid = bcrypt::verify(&payload.password, &teacher.password_hash).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Password verification failed".to_string(),
        )
    })?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    // Generate JWT
    let token = create_jwt(
        teacher.id,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token".to_string(),
        )
    })?;

    Ok(Json(ApiResponse::success(
        "Login successful",
        LoginResponse {
            token,
            teacher: TeacherResponse::from(teacher),
        },
    )))
}

pub async fn logout(
    _auth: AuthenticatedTeacher,
) -> Result<Json<ApiResponse<()>>, (StatusCode, String)> {
    // JWT is stateless, so logout is handled client-side by removing the token
    Ok(Json(ApiResponse {
        success: true,
        message: "Logout successful".to_string(),
        data: None,
    }))
}

pub async fn request_password_reset(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, String)> {
    // Get teacher by email
    let teacher = db::get_teacher_by_email(&state.db, &payload.email)
        .await
        .map_err(|_| {
            // Return success even if email doesn't exist (security best practice)
            (StatusCode::OK, "".to_string())
        })?;

    // Generate reset token
    let reset_token = Uuid::new_v4().to_string();
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

    db::update_reset_token(&state.db, teacher.id, &reset_token, expires_at)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    // In production, send email with reset token
    // For now, we'll just log it (remove in production!)
    tracing::info!(
        "Password reset token for {}: {}",
        payload.email,
        reset_token
    );

    Ok(Json(ApiResponse {
        success: true,
        message: "Password reset instructions sent to email".to_string(),
        data: None,
    }))
}

pub async fn confirm_password_reset(
    State(state): State<AppState>,
    Json(payload): Json<ConfirmResetPasswordRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, String)> {
    // Validate new password
    if payload.new_password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Get teacher by reset token
    let teacher = db::get_teacher_by_reset_token(&state.db, &payload.token)
        .await
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid or expired reset token".to_string(),
            )
        })?;

    // Hash new password
    let password_hash =
        bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to hash password".to_string(),
            )
        })?;

    // Update password and clear reset token
    db::update_password(&state.db, teacher.id, &password_hash)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Password reset successful".to_string(),
        data: None,
    }))
}

pub async fn get_current_teacher(
    State(state): State<AppState>,
    auth: AuthenticatedTeacher,
) -> Result<Json<ApiResponse<TeacherResponse>>, (StatusCode, String)> {
    let teacher = db::get_teacher_by_id(&state.db, auth.teacher_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Teacher not found".to_string()))?;

    Ok(Json(ApiResponse::success(
        "Success",
        TeacherResponse::from(teacher),
    )))
}

// NixOS Config handlers
pub async fn upload_config(
    State(state): State<AppState>,
    auth: AuthenticatedTeacher,
    Json(payload): Json<UploadConfigRequest>,
) -> Result<Json<ApiResponse<ConfigResponse>>, (StatusCode, String)> {
    // Validate filename
    if payload.filename.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Filename cannot be empty".to_string(),
        ));
    }

    // Validate file extension
    if !payload.filename.ends_with(".nix") {
        return Err((
            StatusCode::BAD_REQUEST,
            "File must have .nix extension".to_string(),
        ));
    }

    // Validate content
    if payload.content.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Content cannot be empty".to_string(),
        ));
    }

    let config = db::create_nixos_config(
        &state.db,
        auth.teacher_id,
        &payload.filename,
        &payload.content,
    )
    .await
    .map_err(|e| {
        if e.to_string().contains("unique constraint") {
            (
                StatusCode::CONFLICT,
                "A file with this name already exists".to_string(),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        }
    })?;

    Ok(Json(ApiResponse::success(
        "Configuration file uploaded successfully",
        ConfigResponse::from(config),
    )))
}

pub async fn list_configs(
    State(state): State<AppState>,
    auth: AuthenticatedTeacher,
) -> Result<Json<ApiResponse<Vec<ConfigResponse>>>, (StatusCode, String)> {
    let configs = db::get_teacher_configs(&state.db, auth.teacher_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            )
        })?;

    let responses: Vec<ConfigResponse> = configs.into_iter().map(ConfigResponse::from).collect();

    Ok(Json(ApiResponse::success("Success", responses)))
}

#[axum::debug_handler]
pub async fn get_config(
    State(state): State<AppState>,
    auth: AuthenticatedTeacher,
    Path(config_id): Path<Uuid>,
) -> Result<Json<ApiResponse<ConfigResponse>>, (StatusCode, String)> {
    let config = db::get_config_by_id(&state.db, config_id, auth.teacher_id)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                "Configuration file not found".to_string(),
            )
        })?;

    Ok(Json(ApiResponse::success(
        "Success",
        ConfigResponse::from(config),
    )))
}

pub async fn update_config(
    State(state): State<AppState>,
    auth: AuthenticatedTeacher,
    Path(config_id): Path<Uuid>,
    Json(payload): Json<UpdateConfigRequest>,
) -> Result<Json<ApiResponse<ConfigResponse>>, (StatusCode, String)> {
    // Validate content
    if payload.content.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Content cannot be empty".to_string(),
        ));
    }

    let config = db::update_nixos_config(&state.db, config_id, auth.teacher_id, &payload.content)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                "Configuration file not found".to_string(),
            )
        })?;

    Ok(Json(ApiResponse::success(
        "Configuration file updated successfully",
        ConfigResponse::from(config),
    )))
}

pub async fn delete_config(
    State(state): State<AppState>,
    auth: AuthenticatedTeacher,
    Path(config_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, String)> {
    db::delete_nixos_config(&state.db, config_id, auth.teacher_id)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                "Configuration file not found".to_string(),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Configuration file deleted successfully".to_string(),
        data: None,
    }))
}
