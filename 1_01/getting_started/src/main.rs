use std::{thread, time};
use chrono::{Datelike, Timelike, Utc};
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let downtime = time::Duration::from_millis(5000);
    
    loop { 
        let now = Utc::now();
        println!("{}-{:02}-{:02}-{:02}-{:02}-{:02} {}"
            ,now.year()
            ,now.month()
            ,now.day()
            ,now.hour()
            ,now.minute()
            ,now.second()
            ,s
        );
        thread::sleep(downtime);
    }
}
