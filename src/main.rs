#![allow(non_snake_case)]
use actix_web::{get, web, App, HttpServer, Responder, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[get("/calculateDisselUsageForDistance")]
async fn calculate_dissel_usage_for_distance(
    request: web::Query<CalculatedFuelConsumptionRequest>,
) -> Result<impl Responder> {
    let response = CalculatedFuelConsumptionResponse {
        fuelUsage: request.distance as f32 * request.fuelUsagePer100KM as f32 / 100.0,
    };
    Ok(web::Json(response))
}

#[get("/probabilityOfUnitInjectorFail")]
async fn probability_of_unit_injector_fail(
    _request: web::Query<ProbabilityOfUnitInjectorFailRequest>,
) -> Result<impl Responder> {
    let mut rng = rand::thread_rng();
    let failpercentage = rng.gen_range(0..100);
    let response = ProbabilityOfUnitInjectorFailResponse {
        failProbability: failpercentage as f32 / 100.0,
    };
    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting API for PeopleCar PasWagon C6");
    println!("Listining on 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(calculate_dissel_usage_for_distance)
            .service(probability_of_unit_injector_fail)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Debug, Deserialize)]
pub struct CalculatedFuelConsumptionRequest {
    distance: u32,
    yearOfProduction: u32,
    fuelUsagePer100KM: u32,
}

#[derive(Serialize, Debug)]
pub struct CalculatedFuelConsumptionResponse {
    fuelUsage: f32,
}

#[derive(Debug, Deserialize)]
pub struct ProbabilityOfUnitInjectorFailRequest {
    VIN: String,
}

#[derive(Serialize, Debug)]
pub struct ProbabilityOfUnitInjectorFailResponse {
    failProbability: f32,
}
