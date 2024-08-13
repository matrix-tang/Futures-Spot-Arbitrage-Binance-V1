use crate::conf::C;
use chrono::Local;
use env_logger::Env;
use log::LevelFilter;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

pub fn init_log() {
    let env = Env::default().filter_or("MY_LOG_LEVEL", "debug");

    // 输出到控制台
    let target = if C.log.pattern == "console" {
        env_logger::Target::Stdout
    } else {
        // 输出到文件
        let log_path = format!("{}/{}", C.log.dir, C.log.prefix);
        let t = Box::new(File::create(log_path).expect("Can't create file"));
        env_logger::Target::Pipe(t)
    };

    let level = LevelFilter::from_str(C.log.level.as_str()).unwrap();

    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.f").to_string();
            let level = record.level();
            let module = record.module_path().unwrap_or("<unnamed>");
            let line = record.line().unwrap_or(0);
            let message = record.args();

            writeln!(
                buf,
                "{} [{}] [{}:{}] {}",
                timestamp,
                level,
                module,
                line,
                message
            )
        })
        .target(target)
        .filter(None, level)
        .init();
}
