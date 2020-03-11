use slog::Drain;
use slog_async;
use slog_term;

pub struct Logger {
    pub source_logger: slog::Logger,
}

impl Logger {
    pub fn new() -> Self {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let _log = slog::Logger::root(drain, o!());

        Logger {
            source_logger: _log,
        }
    }
}
