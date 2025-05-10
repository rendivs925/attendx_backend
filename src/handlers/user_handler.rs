use actix_web::{
    HttpRequest, HttpResponse,
    cookie::{Cookie, SameSite, time::Duration},
    web,
};
use log::info;
use serde_json::json;
use std::sync::Arc;
use validator::ValidationErrors;

use crate::{
    constants::COOKIE_NAME,
    services::user_service::UserService,
    types::{
        requests::{
            auth::{login_request::LoginRequest, register_request::RegisterRequest},
            user::update_user_request::UpdateUserRequest,
        },
        responses::api_response::{ApiResponse, ErrorDetails},
    },
    utils::{
        auth_utils::generate_cookie,
        locale_utils::{Messages, Namespace, get_lang},
        validators::{validate_email, validate_name, validate_password},
    },
};

fn handle_validation_error(errors: ValidationErrors, msg: &str) -> HttpResponse {
    let error_details = ErrorDetails {
        details: Some(json!(&errors)),
    };
    HttpResponse::BadRequest().json(ApiResponse::<()>::error(msg, error_details))
}

fn handle_internal_error(err: impl ToString) -> HttpResponse {
    let error_details = ErrorDetails { details: None };
    HttpResponse::InternalServerError()
        .json(ApiResponse::<()>::error(err.to_string(), error_details))
}

fn validate_register_data(
    data: &RegisterRequest,
    messages: &Messages,
) -> Result<(), validator::ValidationErrors> {
    let mut errors = ValidationErrors::new();

    if let Err(e) = validate_name(&data.name, &messages) {
        errors.add("name", e);
    }

    if let Err(e) = validate_email(&data.email, &messages) {
        errors.add("email", e);
    }

    if let Err(e) = validate_password(&data.password, &messages) {
        errors.add("password", e);
    }

    if errors.errors().is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_login_data(
    data: &LoginRequest,
    messages: &Messages,
) -> Result<(), validator::ValidationErrors> {
    let mut errors = ValidationErrors::new();

    if let Err(e) = validate_email(&data.email, &messages) {
        errors.add("email", e);
    }

    if let Err(e) = validate_password(&data.password, &messages) {
        errors.add("password", e);
    }

    if errors.errors().is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub async fn create_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    new_user: web::Json<RegisterRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let data = new_user.into_inner();

    if let Err(errs) = validate_register_data(&data, &messages) {
        let err_msg = messages.get_str(
            Namespace::User,         // Or Validation, depending on where the message is defined
            "register.invalid_data", // Corrected key
            "Invalid registration data",
        );
        return handle_validation_error(errs, &err_msg);
    }

    match user_service.create_user(data, &messages).await {
        Ok(user) => HttpResponse::Created().json(ApiResponse::success(
            messages.get_str(
                Namespace::User,
                "create.success",
                "User successfully created.",
            ),
            user,
        )),
        Err(err) => handle_internal_error(err),
    }
}

pub async fn jwt_login_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    credentials: web::Json<LoginRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);
    let data = credentials.into_inner();

    if let Err(errs) = validate_login_data(&data, &messages) {
        let err_msg = messages.get_str(
            Namespace::User,
            "login.invalid_credentials",
            "Invalid login credentials",
        );
        return handle_validation_error(errs, &err_msg);
    }

    match user_service
        .authenticate_user(&data.email, &data.password, &messages)
        .await
    {
        Ok((user, token)) => {
            info!("User {} successfully logged in.", data.email);
            let cookie = generate_cookie(token);
            HttpResponse::Ok().cookie(cookie).json(ApiResponse::success(
                messages.get_str(Namespace::User, "login.success", "Login successful"),
                user,
            ))
        }
        Err(err) => {
            let error_details = ErrorDetails { details: None };
            HttpResponse::Unauthorized()
                .json(ApiResponse::<()>::error(err.to_string(), error_details))
        }
    }
}

pub async fn logout_user_handler(req: HttpRequest) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    let expired = Cookie::build(&*COOKIE_NAME, "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(Duration::new(0, 0))
        .finish();

    HttpResponse::Ok()
        .cookie(expired)
        .json(ApiResponse::success(
            messages.get_str(
                Namespace::User,
                "logout.success",
                "Logged out successfully.",
            ),
            None::<()>,
        ))
}

pub async fn get_all_users_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match user_service.get_all_users(&messages).await {
        // Pass messages
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_str(
                Namespace::User,
                "fetch.all_success",
                "All users fetched successfully.",
            ),
            users,
        )),
        Err(err) => handle_internal_error(err),
    }
}

pub async fn get_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    email: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match user_service.get_user(&email, &messages).await {
        // Pass messages
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_str(
                Namespace::User,
                "fetch.success",
                "User fetched successfully.",
            ),
            user,
        )),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            messages.get_str(
                Namespace::User,
                "fetch.not_found",
                &format!("User not found: {}", &email), // Include email in default
            ),
            ErrorDetails { details: None },
        )),
        Err(err) => handle_internal_error(err),
    }
}

pub async fn update_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    email: web::Path<String>,
    updated_user: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match user_service
        .update_user(&email, updated_user.into_inner(), &messages) // Pass messages
        .await
    {
        Ok(user) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_str(
                Namespace::User,
                "update.success",
                "User updated successfully.",
            ),
            user,
        )),
        Err(err) => handle_internal_error(err),
    }
}

pub async fn delete_user_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
    email: web::Path<String>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match user_service.delete_user(&email, &messages).await {
        // Pass messages
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_str(
                Namespace::User,
                "delete.success",
                "User deleted successfully.",
            ),
            None::<()>,
        )),
        Err(err) => handle_internal_error(err),
    }
}
