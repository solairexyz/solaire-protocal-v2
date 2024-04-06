#[derive(Serialize, Deserialize)]
struct DeliveryStatusRequest {
    delivery_id: u32,
}

#[post("/delivery/status")]
async fn get_delivery_status(req: web::Json<DeliveryStatusRequest>) -> HttpResponse {
    // Retrieve the delivery status from the database and return it.
    HttpResponse::Ok().json("Delivery status: In transit")
}
