use pyo3::prelude::*;
use std::time::Duration;
use spin_sleep::{SpinSleeper, SpinStrategy};
#[cfg(target_os = "windows")]
use winapi::um::timeapi::{timeBeginPeriod, timeEndPeriod};

/// Ensure Windows high-resolution timing (reduces system timer resolution to 1ms)
#[cfg(target_os = "windows")]
fn enable_high_res_timing() {
    unsafe { timeBeginPeriod(1) };  // Request high-precision timing
}

/// Restore default system timer resolution
#[cfg(target_os = "windows")]
fn disable_high_res_timing() {
    unsafe { timeEndPeriod(1) };  // Reset timer back to normal
}

/// High-precision sleep function
async fn spin_sleep_min(a: f64) {
    #[cfg(target_os = "windows")]
    enable_high_res_timing();  // Apply Windows timer tweak

    let sleeper = SpinSleeper::new(0)  // 0μs native trust → full spin for precision
        .with_spin_strategy(SpinStrategy::SpinLoopHint);  // Use SpinLoopHint for maximum accuracy

    sleeper.sleep(Duration::from_secs_f64(a));

    #[cfg(target_os = "windows")]
    disable_high_res_timing();  // Reset timer to avoid affecting other applications
}

#[pyfunction]
fn sleep(py: Python, a: f64) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        spin_sleep_min(a).await;
        Ok(())
    })
}

#[pymodule]
fn spinsleeper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    Ok(())
}
