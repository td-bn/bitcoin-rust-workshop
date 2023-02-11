use lightning::util::logger::Logger;
use time::OffsetDateTime;

#[derive(Clone, Debug)]
pub struct RLNLogger;

impl Logger for RLNLogger {
    fn log(&self, record: &lightning::util::logger::Record) {
        let raw_log = record.args.to_string();

        let log = format!(
			"{} {:<5} [{}:{}] {}\n",
			OffsetDateTime::now_utc().to_string(),
			record.level.to_string(),
			record.module_path,
			record.line,
			raw_log
		);

        println!("{}", log);
    }
}

