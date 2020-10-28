// use walkdir::WalkDir;


mod fs;
mod shell;

use std::any::type_name;

#[macro_use]
extern crate fstrings;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
//
// fn main() {
//     let mut buffer = String::new();
//     println!("Hello, world!");
//     std::io::stdin().read_line(&mut buffer).expect("Failed");
//     let find_path = buffer.trim();
//     let mut count = 0;
//     // let mut find_path = r"\\192.168.196.99\Diva";
//     let mut find_path2 = r"\\192.168.196.99\Diva";
//     // let mut find_path = buffer.as_str();
//
//     let walker = WalkDir::new(find_path).into_iter();
//     for entry in walker.filter_map(|e| e.ok()) {
//         count += 1;
//         println!("{} - {}", count, entry.path().display());
//     }
//     println!("----- {} = {}  {}",type_of(find_path), type_of(find_path2), find_path2.eq(find_path) );
//     println!("{} = {}", find_path, find_path2);
// }


// use notify::{Watcher, RecursiveMode, watcher};
// use std::sync::mpsc::channel;
// use std::time::Duration;
//
// fn main() {
//     // Create a channel to receive the events.
//     let (sender, receiver) = channel();
//
//     // Create a watcher object, delivering debounced events.
//     // The notification back-end is selected based on the platform.
//     let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();
//
//     // Add a path to be watched. All files and directories at that path and
//     // below will be monitored for changes.
//     watcher.watch(r"C:\Users\ennima.LAPTOP-47QK6MS8\Documents\Develops2020\bicho_data", RecursiveMode::Recursive).unwrap();
//
//     loop {
//         match receiver.recv() {
//            Ok(event) => println!("{:?}", event),
//            Err(e) => println!("watch error: {:?}", e),
//         }
//     }
// }


use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{NaiveDate, NaiveDateTime, Utc, TimeZone, Local, DateTime};
use human_size::{Size, SpecificSize, Kilobyte, Mebibyte, Gigibyte};
use crate::fs::{function, get_file_info};
use crate::shell::{cmd, ffmpeg, StratusTranscoder, TranscoderEngineConfig, StratusTranscoderConfig, FtpLoginConfig};


extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // get_file_info(r"\\192.168.196.99\B_Workflow\StratusArchive\VB2017\AFICION\AF CAMPANIA AFA-NS_DF00A06P.gxf.c485ba");
    cmd();

    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    let ffmpeg_path = env::var("FFMPEG_PATH").unwrap().to_string();
    let ffmpeg_engine = env::var("FFMPEG_ENGINE").unwrap().to_string();
    let media_path = env::var("MEDIA_PAT").unwrap().to_string();
    let media = Path::new(&media_path).join("PADRES CANCER INE_1.flac").to_str().unwrap().to_string();
    // println!("FFMPEG: {:?}", ffmpeg_path.unwrap());
    // let fpath = ffmpeg_path.unwrap().to_string();
    // ffmpeg(&fpath, &media_path);
    let dsplited: Vec<&str> = "ffmpeg -y -i pepa.mp4".split(' ').collect();

    println!("{:?}", dsplited);

    let engine_conf = TranscoderEngineConfig {
        path: ffmpeg_path,
        engine: ffmpeg_engine,
    };

    let stratus_conf = StratusTranscoderConfig {
        ftp_host: "192.168.196.139".to_string(),
        gxf: FtpLoginConfig { user: "movie".to_string(), pass: "".to_string() },
        mxf: FtpLoginConfig { user: "mxfmovie".to_string(), pass: "".to_string() },
        danger: FtpLoginConfig { user: "GVAdmin".to_string(), pass: "AdminGV!".to_string() },
    };

    let transcoder = StratusTranscoder::new(engine_conf, stratus_conf, "TEST".to_string(), media);

    transcoder.transcode();

    Ok(())
}