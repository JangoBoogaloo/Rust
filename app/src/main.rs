use std::fmt::format;

pub trait ILogger {
    fn log(&self);
    fn log_info(&self)
    {
        println!("[Info]");
        self.log();
    }
}

pub trait IFormatter {
    fn format(&self) -> String;
}

pub struct FormattedLogger {
    pub data: String
}

impl IFormatter for FormattedLogger
{
    fn format(&self) -> String {
        self.data.clone()
    }
}

impl ILogger for FormattedLogger
{
    fn log(&self) {
        println!("{}", self.data);
    }
}

fn do_log(logger: &(impl ILogger + IFormatter))
{
    println!("'{}'", logger.format());
    logger.log_info();
}

fn main()
{
    let log = FormattedLogger {
        data: String::from("Hello World")
    };
    do_log(&log);
}