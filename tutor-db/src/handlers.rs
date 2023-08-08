use actix_web::HttpResponse;

pub async fn health_check_handler() -> HttpResponse {
    let response = format!("Alive");
    HttpResponse::Ok().json(&response)
}
