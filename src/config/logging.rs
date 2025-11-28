use tracing_subscriber::EnvFilter;

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("info".parse().unwrap())
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true)
        .compact()
        .init();
}
