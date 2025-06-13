#[cfg(test)]
mod preset {
    use log::debug;
    use serde_json::json;

    use novelai::preset::{model::Model, *};

    fn init_logger() {
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    }

    #[test]
    fn debug_json_v3() {
        init_logger();

        let params = ParametersBuilder::new()
            .v4_prompt(None)
            .v4_negative_prompt(None)
            .build();
        let preset = ImagePresetBuilder::new()
            .model(Model::AnimeV3)
            .parameters(params)
            .build();

        debug!(
            "{:#?}",
            json!({
                "input": preset.prompt,
                "model": preset.model,
                "parameters": preset.parameters,
            })
        );
    }

    #[test]
    fn debug_json_v4() {
        init_logger();

        let preset = ImagePresetBuilder::new().build();

        debug!(
            "{:#?}",
            json!({
              "input": preset.prompt,
              "model": preset.model,
              "parameters": preset.parameters,
            })
        );
    }
}
