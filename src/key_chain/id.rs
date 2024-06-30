use argon2::Config;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use uuid::Uuid;
use x25519_dalek::{PublicKey, StaticSecret};

pub struct MainKey{
    static_secret: StaticSecret,
}

impl MainKey{

    pub fn new(user_name:&str, password:&str ) -> Self {
        Self::get_main_secret(user_name, password)
    }

    pub fn get_main_secret(user_name:&str, password:&str ) -> Self {

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
        let main_secret_s: [u8; 32] = main_secret.clone().try_into().unwrap();
        let secret = StaticSecret::from(main_secret_s);
        return MainKey{
            static_secret: secret
        };
    }

    pub fn get_public_key(&self) -> PublicKey {
        return PublicKey::from(&self.static_secret);
    }

    pub fn get_public_key_as_base64(&self) -> String {
        return BASE64_STANDARD.encode(self.get_public_key())
    }

    pub fn get_uuid(&self) -> Uuid {

        let uuid_0 = Uuid::parse_str("5416127d-98c8-45f2-b216-c616a916384b").unwrap();

        let uuid_1 = Uuid::new_v3(&uuid_0,self.static_secret.as_bytes());

        return uuid_1;
    }

    pub fn get_uuid4config(&self) -> Uuid {

        let uuid_0 = Uuid::parse_str("301cd0db-b144-4d6f-86d1-6f511118e38a").unwrap();

        let uuid_1 = Uuid::new_v3(&uuid_0,self.static_secret.as_bytes());

        return uuid_1;
    }

    pub fn get_sub_key(&self, uuid:&Uuid) -> SubKey {

        let content_str = uuid.as_bytes().as_slice();
        let salt = self.static_secret.as_bytes().as_slice();
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
        let main_secret_s: [u8; 32] = sub_secret.clone().try_into().unwrap();
        let secret = StaticSecret::from(main_secret_s);
        return SubKey{
            static_secret: secret,
            uuid: uuid.clone(),
        };
    }

}

pub struct SubKey{
    static_secret: StaticSecret,
    uuid: Uuid,
}

impl SubKey {

    pub fn get_public_key(&self) -> PublicKey {
        return PublicKey::from(&self.static_secret);
    }

    pub fn get_public_key_as_base64(&self) -> String {
        return BASE64_STANDARD.encode(self.get_public_key())
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_uuid4config(&self ) -> Uuid {

        let uuid_0 = Uuid::parse_str("2d00bf9b-1628-44bf-8133-f6f706544e4f").unwrap();

        let uuid_1 = Uuid::new_v3(&uuid_0,self.static_secret.as_bytes());

        return uuid_1;
    }

}
