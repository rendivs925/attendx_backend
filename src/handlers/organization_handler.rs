use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

use crate::{
    models::organization_model::Organization, services::organization_service::OrganizationService,
};

pub async fn create_organization_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
    organization: web::Json<Organization>,
) -> impl Responder {
    match organization_service
        .create_organization(organization.into_inner())
        .await
    {
        Ok(new_org) => HttpResponse::Created().json(new_org),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_organization_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
) -> impl Responder {
    match organization_service.get_organization_by_id(&org_id).await {
        Ok(Some(organization)) => HttpResponse::Ok().json(organization),
        Ok(None) => HttpResponse::NotFound().body("Organization not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_all_organizations_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
) -> impl Responder {
    match organization_service.get_all_organizations().await {
        Ok(orgs) => HttpResponse::Ok().json(orgs),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_organization_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
    organization: web::Json<Organization>,
) -> impl Responder {
    match organization_service
        .update_organization(&org_id, organization.into_inner())
        .await
    {
        Ok(updated_org) => HttpResponse::Ok().json(updated_org),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_organization_handler(
    organization_service: web::Data<Arc<OrganizationService>>,
    org_id: web::Path<String>,
) -> impl Responder {
    match organization_service.delete_organization(&org_id).await {
        Ok(_) => HttpResponse::Ok().body("Organization deleted successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
