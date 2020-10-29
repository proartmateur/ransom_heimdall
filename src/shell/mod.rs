use std::process::{Command, Stdio};
use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


pub fn cmd() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello>hi.txt"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
}

pub fn ffmpeg(ffmpeg_path: &str, media_path: &str) {
    println!("{}", Path::new(&ffmpeg_path).exists());
    let engine = Path::new(ffmpeg_path).join("ffmpeg.exe");
    let media = Path::new(media_path).join("PADRES CANCER INE_1.flac");
    let out = Command::new(engine)
        .args(&["-i", media.to_str().unwrap(), "au.mp3"])
        .output()
        .expect("failed to execute process");
    println!("{:?}", out);
}

#[derive(Debug)]
pub struct FtpLoginConfig {
    pub user: String,
    pub pass: String,
}

#[derive(Debug)]
pub struct StratusTranscoderConfig {
    pub ftp_host: String,
    pub gxf: FtpLoginConfig,
    pub mxf: FtpLoginConfig,
    pub danger: FtpLoginConfig,
}

#[derive(Debug)]
pub struct TranscoderEngineConfig {
    pub path: String,
    pub engine: String,
}

#[derive(Debug)]
pub struct StratusTranscoder {
    engine: TranscoderEngineConfig,
    config: StratusTranscoderConfig,
    dest_path: String,
    original_media_path: String,
    space: char,
    result_extension: String,
    pub status: bool,
}

//#region Behavior

impl StratusTranscoder {
    fn __engine(&self) -> String {
        let engine = Path::new(&self.engine.path).join(&self.engine.engine).to_str().unwrap().to_string();
        return engine;
    }

    fn __query(&self) -> String {
        let engine = self.__engine();
        let final_extension = self.result_extension.clone();

        let original_media = self.original_media_path.to_string().replace(" ", &self.space.to_string());
        println!("ORIGINAL: {:?}", &original_media);
        let original_media_extension = Path::new(&original_media)
            .extension()
            .and_then(OsStr::to_str).unwrap().to_string();

        let new_media = original_media.clone();
        let new_media = new_media.replace(&original_media_extension, &final_extension);
        let comilla = '"'.to_string();
        let start = f!("-y -i {original_media}");

        let video_query = "-vcodec mpeg2video -vtag xd5b -s 1920x1080 -pix_fmt yuv420p -r 29.97 -rtbufsize 50000k -b:v 50000k -dc 9 -flags +ilme+ildct -top 1".to_string();
        let audio_query = "-acodec pcm_s16le -ac 4".to_string();

        let final_query = f!("{start} {video_query} {audio_query} -f mxf {new_media}");
        // let final_query = f!("{start} {video_query} {audio_query} {new_media}");

        return final_query;
    }

    pub fn new(engine: TranscoderEngineConfig, config: StratusTranscoderConfig, dest_path: String, original_media_path: String) -> Self {
        let space = '❤';
        let result_extension = "mxf".to_string();
        let mut status = false;

        StratusTranscoder {
            engine,
            config,
            dest_path,
            original_media_path,
            space,
            result_extension,
            status,
        }
    }

    pub fn transcode(&mut self) -> bool {
        // println!("Query: {:?}", self.__query());
        let original_media_path = self.original_media_path.clone();
        let media_exists = Path::new(&original_media_path).exists();

        if !media_exists {
            return false;
        }

        let q = self.__query();
        let mut query: Vec<&str> = q.split(" ").collect();
        let video_result = query[query.len() - 1];
        let video_result = video_result.replace(self.space, " ");
        let mut video_result = video_result.as_str().clone();

        let video_input = query[2].clone();
        let video_input = video_input.replace(self.space, " ");
        let video_input = video_input.as_str();

        let mut vr_tmp: String;
        if &video_result == &video_input {
            vr_tmp = video_result.replace(".mxf", "OK.mxf");
            video_result = vr_tmp.as_str().clone();
        }

        let last_index = query.len() - 1;
        let got = std::mem::replace(&mut query[last_index], video_result);
        let got = std::mem::replace(&mut query[2], video_input);
        println!("Query: {:?} last: {:?} are_eq: {:?}", video_input, video_result, video_result == video_input);

        println!("Query: {:?}", &query);

        let engine = self.__engine();
        let out = Command::new(engine)
            .args(&query)
            .output()
            .expect("failed to execute process");
        let exit_status = out.status.code().unwrap().clone();

        if exit_status == 1 {
            return false;
        }
        self.status = true;
        return true;
    }
}

//#endregion