use rts_3d::run;

use log::{info,LevelFilter};
use env_logger::{Builder};

fn main() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();
    info!("Logger initialized");

    run();
}
