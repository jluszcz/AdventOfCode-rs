use env_logger::Target;
use log::LevelFilter;

pub(crate) fn init_logger(level: LevelFilter) -> anyhow::Result<()> {
    init_logger_inner(Some(level), false)
}

pub fn init_test_logger() -> anyhow::Result<()> {
    init_logger_inner(Some(LevelFilter::Trace), true)
}

fn init_logger_inner(level: Option<LevelFilter>, is_test: bool) -> anyhow::Result<()> {
    let _ = env_logger::builder()
        .target(Target::Stdout)
        .filter_level(level.unwrap_or(LevelFilter::Info))
        .is_test(is_test)
        .try_init();

    Ok(())
}
