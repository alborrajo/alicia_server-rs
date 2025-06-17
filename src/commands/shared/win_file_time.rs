use deku::{DekuRead, DekuWrite};

#[derive(Default, Debug, DekuRead, DekuWrite)]
pub struct WinFileTime {
    pub low_date_time: u32,
    pub high_date_time: u32,
}
impl WinFileTime {
    pub fn from_unix_time(unix_time: i64) -> Self {
        let seconds = unix_time / 1_000_000;
        let nanos = (unix_time % 1_000_000) * 10;

        // Windows file time is in 100-nanosecond intervals since January 1, 1601
        let windows_epoch = 11644473600; // seconds from Unix epoch to Windows epoch
        let total_seconds = seconds + windows_epoch;
        
        let low_date_time = (total_seconds as u64 * 10_000_000 + nanos as u64) as u32;
        let high_date_time = ((total_seconds as u64 * 10_000_000 + nanos as u64) >> 32) as u32;

        WinFileTime {
            low_date_time,
            high_date_time,
        }
    }

    pub fn to_unix_time(&self) -> i64 {
        let total_time = ((self.high_date_time as u64) << 32) | (self.low_date_time as u64);
        let total_seconds = total_time / 10_000_000;
        let nanos = (total_time % 10_000_000) / 10;

        // Convert back to Unix time
        let unix_epoch = 11644473600; // seconds from Windows epoch to Unix epoch
        total_seconds as i64 - unix_epoch + nanos as i64 / 1_000_000
    }
}