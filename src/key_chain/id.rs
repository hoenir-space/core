use argon2::{self, Config};
use uuid::Uuid;

pub fn get_main_secret(user_name:&str, password:&str ) -> Vec<u8> {

    let salt_uuid = Uuid::parse_str("ae95988c-f542-4db7-bf49-e62e7583c356").unwrap();
    let content_str = format!("{} . {}", password, user_name);
    let salt = salt_uuid.as_bytes().as_slice();
    let salt_config = Config {
        hash_length: 256,
        .. Config::default()
    };
    let salt_hash = argon2::hash_raw(content_str.as_bytes(), salt, &salt_config).unwrap();



    let base_str = format!("{} . {}", user_name, password);
    let salt = salt_hash.as_slice();
    let config = Config {
        hash_length: 32,
        .. Config::default()
    };
    let main_secret = argon2::hash_raw(base_str.as_bytes(), salt, &config).unwrap();
    return main_secret;
}


pub fn get_sub_secret(uuid:&Uuid, main_secret:&Vec<u8> ) -> Vec<u8> {

    let content_str = uuid.as_bytes().as_slice();
    let salt = main_secret.as_slice();
    let salt_config = Config {
        hash_length: 256,
        .. Config::default()
    };
    let salt_hash = argon2::hash_raw(content_str, salt, &salt_config).unwrap();



    let base_str = uuid.as_bytes().as_slice();
    let salt = salt_hash.as_slice();
    let config = Config {
        hash_length: 32,
        .. Config::default()
    };
    let sub_secret = argon2::hash_raw(base_str, salt, &config).unwrap();
    return sub_secret;
}

pub fn get_uuid4identity(main_secret:&Vec<u8> ) -> Uuid {

    let uuid_0 = Uuid::parse_str("5416127d-98c8-45f2-b216-c616a916384b").unwrap();

    let uuid_1 = Uuid::new_v3(&uuid_0,main_secret);

    return uuid_1;
}

pub fn get_uuid4config(main_secret:&Vec<u8> ) -> Uuid {

    let uuid_0 = Uuid::parse_str("301cd0db-b144-4d6f-86d1-6f511118e38a").unwrap();

    let uuid_1 = Uuid::new_v3(&uuid_0,main_secret);

    return uuid_1;
}
