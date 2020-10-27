use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{NaiveDate, NaiveDateTime, Utc, TimeZone, Local, DateTime};
use human_size::{Size, SpecificSize, Kilobyte, Mebibyte, Gigibyte, Kibibyte, Pebibyte, Byte};
use std::any::Any;


pub fn function() {
    println!("called `my::function()`");
}

pub fn get_date(f_time: SystemTime) -> DateTime<Local> {
    let timestamp = get_timestamp(f_time);

    return get_date_fom_timestamp(timestamp);
}

pub fn get_date_fom_timestamp(timestamp: u64) -> DateTime<Local> {
    let utc = Utc.timestamp(timestamp as i64, 0);
    let converted: DateTime<Local> = DateTime::from(utc);
    return converted;
}
pub fn get_timestamp(f_time: SystemTime) -> u64 {
    return f_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
}

#[derive(Copy, Clone, Debug)]
pub struct FileInfo {
    created: u64,
    modified: u64,
    size: u64,
}

impl FileInfo {
    pub fn size_human(&self) -> String {
        let val = self.size_PB().value();
        if val >= 1.0 {
            return self.size_PB().to_string();
        }

        let val = self.size_GB().value();
        if val >= 1.0 {
            return self.size_GB().to_string();
        }

        let val = self.size_MB().value();
        if val >= 1.0 {
            return self.size_MB().to_string();
        }

        let val = self.size_KB().value();
        if val >= 1.0 {
            return self.size_KB().to_string();
        }

        let val = self.size_B().value();
        if val >= 1.0 {
            return self.size_B().to_string();
        }

        println!("PB: {:?}", val);
        return val.to_string();
    }

    pub fn size_B(&self) -> SpecificSize<Byte> {
        let size_B = self.size.to_string() + " B";
        let size_Bi: SpecificSize<Byte> = size_B.parse().unwrap();
        return size_Bi;
    }

    pub fn size_KB(&self) -> SpecificSize<Kibibyte> {
        let size_B = self.size.to_string() + " B";
        let size_KB: SpecificSize<Kibibyte> = size_B.parse().unwrap();
        return size_KB;
    }

    pub fn size_MB(&self) -> SpecificSize<Mebibyte> {
        let size_B = self.size.to_string() + " B";
        let size_MB: SpecificSize<Mebibyte> = size_B.parse().unwrap();
        return size_MB;
    }

    pub fn size_GB(&self) -> SpecificSize<Gigibyte> {
        let size_B = self.size.to_string() + " B";
        let size_GB: SpecificSize<Gigibyte> = size_B.parse().unwrap();
        return size_GB;
    }

    pub fn size_PB(&self) -> SpecificSize<Pebibyte> {
        let size_B = self.size.to_string() + " B";
        let size_PB: SpecificSize<Pebibyte> = size_B.parse().unwrap();
        return size_PB;
    }

    pub fn created_local(&self) -> DateTime<Local> {
        return get_date_fom_timestamp(self.created);
    }

    pub fn modified_local(&self) -> DateTime<Local> {
        return get_date_fom_timestamp(self.modified);
    }
}

pub fn get_file_info(file_path: &str) {
    // use std::fs;
    let metadata = std::fs::metadata(file_path).unwrap();
    println!("{:?}", metadata);
    println!("Modified: {:?}", get_date(metadata.modified().unwrap()));
    println!("Accessed: {:?}", get_date(metadata.accessed().unwrap()));

    let size_f2 = metadata.len().to_string() + " B";
    let size_f_gb: SpecificSize<Gigibyte> = size_f2.parse().unwrap();
    println!("Size: {:?}", size_f_gb);
    let created = get_timestamp(metadata.modified().unwrap());
    let modified = get_timestamp(metadata.modified().unwrap());
    let info = FileInfo { created, modified, size: (metadata.len()) };

    println!("Info: {:?}", info);
    println!("Info Size: {:?}", info.size_GB());
    println!("Info Human Size: {:?}", info.size_human());
    println!("Info Created: {:?} -- {:?}", info.created, info.created_local());
}