use std::io::BufReader;
use std::time::Duration;
use std::sync::mpsc;
use std::collections::HashMap;
use std::thread;
use mp3_duration;
use std::path::Path;
use std::time::Instant;
mod include{
    pub mod ncplayer;
}
pub use include::ncplayer::*;
