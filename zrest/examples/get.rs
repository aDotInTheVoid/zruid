
// For a local development instance, dont waste your time
const ZOE_API_KEY: &str = "XNKVLPJXdCHDp2WcXBGHWtAVAYjZIMVJ";
const ZOE_EMAIL: &str = "ZOE@zulip.com";
const DEV_SERVER: &str = "localhost:9991";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let c = zrest::Client::new(ZOE_EMAIL, ZOE_API_KEY, DEV_SERVER);
    
    match c.users().await {
        Ok(m) => println!("OK:",),
        Err(e) => println!("ERR: {:?}", e)
    }

    match c.send("Hello joe").await {
        Ok(m) => println!("OK: {}", m),
        Err(e) => println!("ERR: {:?}", e)
    }

    
    
}