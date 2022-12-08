use std::time::Duration;

use fast_log::{
    consts::LogSize,
    plugin::{file_split::RollingType, packer::ZipPacker},
    Config,
};

use crate::CONTEXT_MANAGER;
use itoo_config::ApplicationConfig;

//日志配置初始化
pub fn init_log() {
    let config = CONTEXT_MANAGER.get::<ApplicationConfig>();
    //日志目录创建
    std::fs::create_dir_all(&config.log.dir);
    fast_log::init(
        Config::new()
            .console()
            .file_split(
                &config.log.dir,
                str_to_temp_size(&config.log.size),
                str_to_rolling(&config.log.rolling_type),
                ZipPacker {},
            )
            .level(str_to_log_level(&config.log.level)),
    )
    .unwrap();
}

fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].parse::<usize>().unwrap();
            LogSize::MB(num)
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].parse::<usize>().unwrap();
            LogSize::KB(num)
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].parse::<usize>().unwrap();
            LogSize::GB(num)
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].parse::<i64>().unwrap();
            RollingType::KeepNum(num)
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].parse::<u64>().unwrap();
            RollingType::KeepTime(Duration::from_secs(num))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> log::LevelFilter {
    match arg {
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        "trace" => log::LevelFilter::Trace,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        _ => log::LevelFilter::Off,
    }
}
