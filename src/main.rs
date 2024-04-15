mod utils;
mod controllers;
mod routes;
mod middleware;

use crate::utils::asyncfn;
use crate::utils::cache;

#[tokio::main]
async fn main() { 
    // asyncfn::run().await;
    cache::run();
}
