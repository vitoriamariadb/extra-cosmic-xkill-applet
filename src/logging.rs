use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;

use once_cell::sync::Lazy;

static LOGGER: Lazy<AppLogger> =
    Lazy::new(|| AppLogger::new().expect("Falha ao inicializar logger"));

pub struct AppLogger {
    log_path: PathBuf,
    file: Mutex<fs::File>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

impl AppLogger {
    pub fn new() -> anyhow::Result<Self> {
        let log_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("extra-cosmic-xkill")
            .join("logs");
        fs::create_dir_all(&log_dir)?;

        let log_path = log_dir.join("xkill-applet.log");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        Ok(Self {
            log_path,
            file: Mutex::new(file),
        })
    }

    pub fn log(&self, level: LogLevel, module: &str, message: &str) {
        let timestamp = format_timestamp();
        let line = format!(
            "[{}] [{}] {}: {}\n",
            timestamp,
            level.as_str(),
            module,
            message
        );

        if let Ok(mut file) = self.file.lock() {
            let _ = file.write_all(line.as_bytes());
        }

        match level {
            LogLevel::Error => eprintln!("{}", line.trim()),
            _ => println!("{}", line.trim()),
        }
    }

    pub fn log_path(&self) -> &PathBuf {
        &self.log_path
    }

    pub fn rotate(&self, max_size: u64) -> anyhow::Result<()> {
        let metadata = fs::metadata(&self.log_path)?;
        if metadata.len() > max_size {
            let backup = self.log_path.with_extension("log.old");
            fs::rename(&self.log_path, &backup)?;
            if let Ok(mut file) = self.file.lock() {
                *file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.log_path)?;
            }
        }
        Ok(())
    }
}

fn format_timestamp() -> String {
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let hours = (secs / 3600) % 24;
    let minutes = (secs / 60) % 60;
    let seconds = secs % 60;
    let days = secs / 86400;
    let year = 1970 + days / 365;
    let remaining_days = days % 365;
    let month = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

pub fn app_log(level: LogLevel, module: &str, message: &str) {
    LOGGER.log(level, module, message);
}

pub fn info(module: &str, message: &str) {
    app_log(LogLevel::Info, module, message);
}

pub fn warn(module: &str, message: &str) {
    app_log(LogLevel::Warn, module, message);
}

pub fn error(module: &str, message: &str) {
    app_log(LogLevel::Error, module, message);
}

pub fn debug(module: &str, message: &str) {
    app_log(LogLevel::Debug, module, message);
}

// "A alma que se tornou consciente de si mesma nao busca aprovacao externa." - Epicteto
