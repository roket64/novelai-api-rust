use novelapi_lib::{client::NAIClient, preset::ImagePresetBuilder};

pub async fn generate_image_v4_example() {
    dotenv::dotenv().ok();

    let username = dotenv::var("USERNAME").unwrap();
    let password = dotenv::var("PASSWORD").unwrap();

    let client = NAIClient::new(&username, &password).await;

    let preset = ImagePresetBuilder::new()
        .prompt(String::from("1girl"))
        .build();

    let _ = client.generate_image(preset).await;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    generate_image_v4_example().await;
}
