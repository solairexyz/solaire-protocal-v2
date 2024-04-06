use actix_web::{web, post, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct PurchaseRequest {
    product_id: u32,
    user_id: u32,
    quantity: u32,
}

#[post("/buy")]
async fn buy_product(req: web::Json<PurchaseRequest>) -> HttpResponse {
    // Validate the request, check product availability, and update the database.
    // For simplicity, this example assumes the purchase is always successful.
    HttpResponse::Ok().json("Purchase successful")
}
