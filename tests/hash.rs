#[cfg(test)]
mod hash {
  use log::debug;
  use novelai::utils::hash::encode_access_key;

  fn init_logger() {
      log4rs::init_file("log4rs.yml", Default::default()).unwrap();
  }

  #[test]
  fn debug_access_key() {
    init_logger();

    let key = encode_access_key("username", "password");
    debug!("key: {:#?}", key);
  }
}