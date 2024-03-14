use crate::conf::C;
use chrono::Local;
use env_logger::fmt::Color;
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
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug | log::Level::Trace => Color::Cyan,
            };

            let mut level_style = buf.style();
            level_style.set_color(level_color).set_bold(true);

            let mut style = buf.style();
            style.set_color(Color::White).set_dimmed(true);

            writeln!(
                buf,
                "{} {} [ {}:{} ] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.f"),
                level_style.value(record.level()),
                style.value(record.module_path().unwrap_or("<unnamed>")),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .target(target)
        .filter(None, level)
        .init();
}
