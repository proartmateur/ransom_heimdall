// use walkdir::WalkDir;
mod fs;

use std::any::type_name;

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

fn main() -> std::io::Result<()> {
    // use std::fs;

    get_file_info(r"\\192.168.196.99\B_Workflow\StratusArchive\VB2017\AFICION\AF CAMPANIA AFA-NS_DF00A06P.gxf.c485ba");
    // let metadata = fs::metadata(r"\\192.168.196.99\B_Workflow\StratusArchive\VB2017\AFICION\AF COMENTARIO ERIKA-FT_DF00A06X.gxf")?;
    //
    // println!("{:?}", metadata.file_type());
    // println!("{:?}", metadata);
    // let f_created:u64 = metadata.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
    // println!("{:?}", metadata.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs());
    //
    // let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(f_created as i64, 0);
    // println!("{}", date_time_after_a_billion_seconds);
    // println!("{:?}", Utc.timestamp(f_created as i64, 0).to_string());
    //
    //
    // let utc = Utc::now();
    // let local = Local::now();
    // let converted: DateTime<Local> = DateTime::from(utc);
    // let converted2: DateTime<Local> = DateTime::from(Utc.timestamp(f_created as i64, 0));
    // // println!("{:?}  -- {:?}  ---  {:?}", local, utc, converted);
    // println!("{:?}", converted2);
    // let size_f= metadata.len().to_string() + " B";
    // let size_f:Size = size_f.parse().unwrap();
    //
    // println!("{:?}    {:?}",type_of(metadata.len()), metadata.len() );
    // println!("{:?}", size_f.to_string());
    // let size_f2= metadata.len().to_string() + " B";
    // let size_f_kb: SpecificSize<Gigibyte> = size_f2.parse().unwrap();
    // println!("{:?}", size_f_kb.to_string());
    //
    // function();

    Ok(())
}