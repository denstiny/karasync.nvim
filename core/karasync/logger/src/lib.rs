use fern::log_file;
pub use log::{debug, error, info, warn};

#[allow(dead_code)]
pub trait HandleResult<T, E> {
    fn expect_log(self, ok_handler: fn(T), warn_handler: fn(E));
}

#[macro_export]
macro_rules! handle_log {
    ($leve:expr,$error_message:expr) => {
        log!($leve, "{}", $error_message)
    };
}

impl<T, E> HandleResult<T, E> for Result<T, E> {
    fn expect_log(self, ok_handler: fn(T), err_handler: fn(E)) {
        match self {
            Ok(value) => ok_handler(value),
            Err(e) => err_handler(e),
        }
    }
}

pub fn init_logger(file_path: &str) {
    // use builder methods
    use fern::colors::{Color, ColoredLevelConfig};

    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    let colors_level = colors_line.info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{target}]{color_line}[{level}] {color_line}{message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message
            ))
        })
        .chain(std::io::stdout())
        .chain(log_file(file_path).unwrap())
        .apply()
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use log::{debug, error, info, warn};

    #[allow(dead_code)]
    fn fn_macor(i: i32) -> Result<String, String> {
        if i == 1 {
            return Err(String::from("helo"));
        }
        Ok(String::from("world"))
    }

    #[test]
    fn it_works() {
        init_logger("log.txt");
        info!("test info message");
        error!("test error message");
        debug!("teest debug message");
        warn!("test warn message");
    }
}
