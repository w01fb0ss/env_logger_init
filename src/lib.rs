use chrono::Local;
use std::env;
use std::io::Write;
use std::sync::Once;

static LOGGER_INIT: Once = Once::new();

pub fn init(level: &str) {
    let level = match env::var("RUST_LOG") {
        Err(_) => level.to_owned(),
        Ok(value) => value,
    };
    env::set_var("RUST_LOG", &level);

    LOGGER_INIT.call_once(|| {
        let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
        env_logger::Builder::from_env(env)
            .format(|buf, record| {
                let level = { buf.default_styled_level(record.level()) };
                let file = match record.file() {
                    Some(file) => file.strip_prefix("src/").unwrap_or("<unnamed>"),
                    None => "<unnamed>",
                };
                writeln!(
                    buf,
                    "{} {} [{}::{}({})] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    level,
                    record.module_path().unwrap_or("<unnamed>"),
                    file,
                    record.line().unwrap_or(0),
                    &record.args(),
                )
            })
            .init();
    });
}
