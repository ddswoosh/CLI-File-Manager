mod utils;
mod controllers;
mod routes;
mod middleware;

use crate::utils::asyncfn;

#[tokio::main]
async fn main() { 
    asyncfn::run().await;
}
