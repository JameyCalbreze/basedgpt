use log;
use log::{LevelFilter, Metadata, Record};

static LOGGER: BasedGptLog = BasedGptLog;

struct BasedGptLog;

impl log::Log for BasedGptLog {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
}

