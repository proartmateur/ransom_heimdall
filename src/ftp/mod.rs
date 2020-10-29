use ftp::FtpStream;
pub fn ftp_test() {
    let mut ftp_stream = FtpStream::connect("192.168.196.139:21").unwrap_or_else(|err|
        panic!("{}", err)
    );
    let _ = ftp_stream.quit();
}
