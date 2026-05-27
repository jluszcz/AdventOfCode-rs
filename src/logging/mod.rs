use env_logger::Target;
use log::LevelFilter;

pub(crate) fn init_logger(level: LevelFilter) -> anyhow::Result<()> {
    init_logger_inner(level, false)
}

pub fn init_test_logger() -> anyhow::Result<()> {
    init_logger_inner(LevelFilter::Trace, true)
}

fn init_logger_inner(level: LevelFilter, is_test: bool) -> anyhow::Result<()> {
    let _ = env_logger::builder()
        .target(Target::Stdout)
        .filter_level(level)
        .is_test(is_test)
        .try_init();

    Ok(())
}
