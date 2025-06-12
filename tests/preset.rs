#[cfg(test)]
mod preset {
    use log::debug;
    use novelapi_lib::preset::*;
    use serde_json::json;

    fn init_logger() {
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    }

    #[test]
    fn debug_builder() {
        init_logger();
        debug!("{:#?}", ImagePresetBuilder::new());
    }

    #[test]
    fn debug_preset() {
        debug!("{:#?}", ImagePresetBuilder::new().build());
    }

    #[test]
    fn debug_json() {
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
