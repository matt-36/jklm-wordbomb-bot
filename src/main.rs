use std::error::Error;


type Result<T> = Result<T, Box<dyn Error>>; 

enum Codes {
    InitRecv = 0, // first game data
    HeartbeatRecv = 2, // server heartbeat
    HeartbeatSend = 3, // sent after heartbeat recv
    InitSend = 40, // num for first payload send
    JoinRoom = 420, // num for chatroom join
    JoinedRoom = 430, // recv that player joined room
    Data = 42, // all game related data transfer
}

struct Game {
    words_used: Vec<String>,
    curr_letters: String,
}

async fn join_match(code: String) -> Result<()> {
    // send request to https://jklm.fun/api/joinRoom with data {roomCode: code}
}

async fn run() -> Result<()> {
    if let Ok(_) = join_match("".to_string()).await { // tell server who we are/what we want to connect to
        // start ws connection to wss://phoenix.jklm.fun/socket.io/?EIO=4&transport=websocket
        //
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

