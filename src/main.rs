
use std::{sync::Mutex};
use std::borrow::Cow;
use rust_socketio::ClientBuilder;
use serde_json::json;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use rust_socketio::{Client, Payload};

const WORDS_BYTES: &[u8] = std::include_bytes!("../target/words");

lazy_static::lazy_static! {
    static ref WORDS: Vec<Cow<'static, str>> = {
        WORDS_BYTES
            .split(|a|*a==b'\n')
            .map(|word|Cow::Borrowed(std::str::from_utf8(word).unwrap()))
            .collect()
    };
    static ref WORDS_MAPPED: Arc<RwLock<HashMap<String, Arc<Vec<Cow<'static, str>>>>>> = Default::default();
}

fn load_words(starting_with: String) -> Arc<Vec<Cow<'static, str>>> {
    if let Some(a) = WORDS_MAPPED.read().unwrap().get(&starting_with) {
        a.clone()
    } else {
        let v = Arc::new(WORDS.iter().filter_map(|a|if a.contains(&starting_with) {
            Some(a.clone())
        } else {
            None
        }).collect::<Vec<_>>());
        WORDS_MAPPED.write().unwrap().insert(starting_with, v.clone());
        v
    }
}

struct Game {
    code: String,
}

impl Game {
    fn start(token: String) {

    }
}

fn on_setup(payload: Payload, client: Client) {
    println!("{:?}", &payload)
}

fn on_next_turn(payload: Payload, client: Client) {
    println!("{:?}", payload);
}

fn on_correct_word(payload: Payload, client: Client) {

}

fn on_fail_word(payload: Payload, client: Client) {

}

fn main() {
    let code = "";
    let token = "";

    let socket = ClientBuilder::new("https://phoenix.jklm.fun")
        .namespace("/socket.io")
        .on("setup", on_setup)
        .on("nextTurn", on_next_turn)
        .on("correctWord", on_correct_word)
        .on("failWord", on_fail_word)
        .connect()
        .expect("failed to connect to socket");
    
    socket.emit("joinGame", json!(["bombparty", code, token])).unwrap();


}

