#[derive(Serialize, Deserialize)]
struct SellRequest {
    user_id: u32,
    product_name: String,
    description: String,
    price: f32,
    token_address: String,
    contract_address: String,
}

#[post("/sell")]
async fn sell_product(req: web::Json<SellRequest>) -> HttpResponse {
    // Add the product to the database and return a success response.
    HttpResponse::Ok().json("Product listed for sale")
}
