use uuid::Uuid;
use crate::key_chain::id::{MainKey};

pub fn run(){

    let main_key = MainKey::new("test@example.com","pa55w0rd");

    println!("uuid:        {}", main_key.get_uuid());
    println!("config uuid: {}", main_key.get_uuid4config());
    println!();
    let rnd_uuid = Uuid::new_v4();
    let sub_key = main_key.get_sub_key(&rnd_uuid);
    println!("uuid:        {}", sub_key.get_uuid());
    println!("pub key:     {}", sub_key.get_public_key_as_base64());
    println!("config uuid: {}", sub_key.get_uuid4config());
}