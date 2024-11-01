use chrono::Local;
use log::{Level, Metadata, Record};
use serde_json::json;
use std::thread;

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // ใช้ chrono เพื่อดึงวันเวลาในรูปแบบ YYYY/MM/DD hh:mm:ss:sss
            let now = Local::now();
            let formatted_date = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

            // ดึงชื่อ thread ปัจจุบัน
            let thread_name = thread::current().name().unwrap_or("unknown").to_string();

            // สร้าง JSON object
            let log_json = json!({
                "Timestamp": formatted_date,
                "Level": record.level().to_string(),
                "Thread": thread_name,
                "Target": record.target(),
                "Message": record.args().to_string(),
            });

            // แสดง JSON object
            println!("{}", log_json.to_string());
        }
    }

    fn flush(&self) {}
}