use base64::{self, prelude::BASE64_URL_SAFE, Engine};

const DATA_ACCESS_IDENT: &str = "novelai_data_access_key";

// https://github.com/Aedial/novelai-api/blob/main/novelai_api/utils.py
pub fn encode_access_key(username: &str, password: &str) -> String {
    let pre_salt = format!("{}{}{}", &password[..6], &username, &DATA_ACCESS_IDENT);

    let mut salt_buf = [0u8; 16];
    let mut key_buf = [0u8; 64];

    let hash_blake2 = |data: &[u8], buffer: &mut [u8]| {
        use blake2::digest::{Update, VariableOutput};

        let mut blake2 = blake2::Blake2bVar::new(16usize).unwrap();
        blake2.update(data);
        blake2.finalize_variable(buffer).unwrap();
    };

    let hash_argon2 = |data: &[u8], salt: &[u8], buffer: &mut [u8]| {
        use argon2::{Algorithm::Argon2id, Argon2, ParamsBuilder, Version::V0x13};

        let argon2 = Argon2::new(
            Argon2id,
            V0x13,
            ParamsBuilder::new()
                .t_cost(2)
                .m_cost(1953u32)
                .p_cost(1)
                .output_len(64usize)
                .build()
                .unwrap(),
        );
        let _ = argon2.hash_password_into(data, salt, buffer).unwrap();
    };

    hash_blake2(&pre_salt.as_bytes(), &mut salt_buf);
    hash_argon2(&password.as_bytes(), &salt_buf, &mut key_buf);

    let key = BASE64_URL_SAFE.encode(key_buf)[..64].to_string();
    key
}
