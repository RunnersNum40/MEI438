#[macro_export]
macro_rules! log_duration {
    ($name:expr, $block:block) => {{
        #[cfg(debug_assertions)]
        {
            use std::time::Instant;
            let start = Instant::now();
            let result = $block;
            let duration = start.elapsed();
            log::info!("{} took {:?}", $name, duration);
            result
        }
        #[cfg(not(debug_assertions))]
        {
            $block
        }
    }};
}
