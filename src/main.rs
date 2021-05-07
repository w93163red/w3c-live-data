mod data;
mod draw;

use anyhow::Result;
use data::fetch::Data;
use draw::draw;
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    let tick_rate = Duration::from_millis(5000);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            if last_tick.elapsed() >= tick_rate {
                tx.send(0).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let mut data = Data::new();
    loop {
        data.fetch_ongoing_match();
        draw(&data)?;
        rx.recv()?;
    }
}
