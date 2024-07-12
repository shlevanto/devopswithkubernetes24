use std::env;
use dotenv::dotenv;


pub fn port() -> u16 {
    dotenv().ok();
    let port: u16 = env::var("PORT").unwrap().parse::<u16>().unwrap();
    return port;
}
