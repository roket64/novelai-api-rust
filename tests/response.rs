#[cfg(test)]
mod response {
    use log::debug;

    #[tokio::test]
    async fn debug_response_v3() {
        use novelapi_lib::client::NAIClient;

        use novelapi_lib::preset::model::Model::AnimeV3;
        use novelapi_lib::preset::ImagePresetBuilder;
        use novelapi_lib::preset::ParametersBuilder;

        log4rs::init_file("log4rs.yml", Default::default()).unwrap();

        let params = ParametersBuilder::new()
            .v4_prompt(None)
            .v4_negative_prompt(None)
            .build();
        let preset = ImagePresetBuilder::new()
            .model(AnimeV3)
            .parameters(params)
            .build();

        dotenv::dotenv().ok();

        let uesrname = dotenv::var("USERNAME").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();

        let client = NAIClient::new(&uesrname, &password).await;

        let response = client.send_request(preset).await;

        assert!(&response.status().is_success());

        debug!("{:#?}", &response);
    }

    #[tokio::test]
    async fn debug_response_v4() {
        use novelapi_lib::client::NAIClient;

        use novelapi_lib::preset::ImagePresetBuilder;
        use novelapi_lib::preset::ParametersBuilder;

        log4rs::init_file("log4rs.yml", Default::default()).unwrap();

        let preset = ImagePresetBuilder::new()
            .parameters(ParametersBuilder::new().build())
            .build();

        dotenv::dotenv().ok();

        let uesrname = dotenv::var("USERNAME").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();

        let client = NAIClient::new(&uesrname, &password).await;

        let response = client.send_request(preset).await;

        assert!(&response.status().is_success());

        debug!("{:#?}", &response);
    }
}
