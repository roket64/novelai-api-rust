use dotenv;
use log::debug;
use log4rs::{self};
use novelapi_lib::schema::uc_preset::*;
use novelapi_lib::utils::config::{ImageConfigBuilder, ParametersBuilder};
use serde_json::json;
use std::default::Default;
use std::env;

use novelapi_lib::client::NAIClient;

const IMAGE_QUALITY_TAGS: &'static str = "best quality, amazing quality, very aesthetic, absurdres";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let username = env::var("USERNAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let client = NAIClient::new(&username, &password).await;
    debug!("{:#?}", client);

    let params = ParametersBuilder::new()
        .steps(23u32)
        .seed(12345u32)
        .scale(5.0f32)
        .negative_prompt(V3_HEAVY.to_string())
        .build();
    debug!("parameters built: {:#?}", params);

    // let prompt = "solo, 1female, standing, full_body";
    let prompt = "";

    let cfg = ImageConfigBuilder::new(format!("{}, {}", prompt, IMAGE_QUALITY_TAGS).as_str())
        .params(params)
        .build();
    debug!("{:#?}", cfg);

    let json = json!({
        "input": cfg.prompt,
        "model": cfg.model,
        "action": cfg.action,
        "parameters": cfg.parameters,
    });
    debug!("{:#?}", json);

    client.generate_image_from(cfg).await;
}
