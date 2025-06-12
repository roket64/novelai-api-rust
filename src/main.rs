#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}
