use log::{info, warn,debug,trace};   

use std::time::SystemTime;
use fern::colors::{Color, ColoredLevelConfig};


fn html_color(level: &log::Level) -> &'static str {
    match level {
        log::Level::Error => "<span style=\"color:Red\">Error</span>",
        log::Level::Warn => "<span style=\"color:Yellow\">Warn</span>",
        log::Level::Info => "<span style=\"color:Green\">Info</span>",
        log::Level::Debug => "<span style=\"color:Magenta\">Debug</span>",
        log::Level::Trace => "<span style=\"color:Cyan\">Trace</span>",
    }
}
fn setup_logging() {
    
    let base_config = fern::Dispatch::new();
    let colors = ColoredLevelConfig::new().debug(Color::Magenta).warn(Color::Yellow).info(Color::Green).trace(Color::Cyan).error(Color::Red);
    let stdout_config = fern::Dispatch::new()
    .chain(std::io::stdout())
    .format(move |out, message, record| {
        out.finish(format_args!(
            "[{}] <{}:{}> {}\n\r",
            // just use 'colors.color(..)' instead of the level
            // itself to insert ANSI colors.
            colors.color(record.level()),
            record.target(),
            humantime::format_rfc3339_seconds(SystemTime::now()),
            message,
        ))
    });

    let file_config = fern::Dispatch::new()
    .format(move |out, message, record| {
        out.finish(format_args!(
            "[{}] &#60;{}:{}&#62; {}<br>",
            // just use 'colors.color(..)' instead of the level
            // itself to insert ANSI colors.
            html_color(&record.level()),
            record.target(),
            humantime::format_rfc3339_seconds(SystemTime::now()),
            message,
        ))
    })
    .chain(fern::log_file("program.html").unwrap());
    
    let _ =  base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply();

}

fn main() {

    setup_logging();
    warn!("This is a warning message");
    info!("This is an info message");
    debug!(" {} ",2);  
    trace!("This is a trace message");
} 