use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};

use crate::{
    services::user_service::UserService,
    types::{
        requests::user::update_user_request::UpdateUserRequest,
        responses::api_response::{ApiResponse, ErrorDetails},
    },
    utils::{
        locale_utils::{Messages, get_lang},
        validation_utils::handle_internal_error,
    },
};

pub async fn get_all_users_handler(
    req: HttpRequest,
    user_service: web::Data<Arc<UserService>>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let messages = Messages::new(lang);

    match user_service.get_all_users(&messages).await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_user_message("fetch.all_success", "All users fetched successfully."),
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
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_user_message("fetch.success", "User fetched successfully."),
            user,
        )),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            messages.get_user_message("fetch.not_found", &format!("User not found: {}", &email)),
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
        .update_user(&email, updated_user.into_inner(), &messages)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_user_message("update.success", "User updated successfully."),
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
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success(
            messages.get_user_message("delete.success", "User deleted successfully."),
            None::<()>,
        )),
        Err(err) => handle_internal_error(err),
    }
}
