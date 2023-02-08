use std::error::Error;
use std::process;
use std::sync::Arc;
use tokio::signal::{self, unix::SignalKind};
use tokio::sync::Mutex;
use tokio::time::Duration;

/// Represents a single background task that runs for a specified duration.
/// The `value` argument is a shared `Arc<Mutex<u64>>` that is updated with the value of `seconds`
/// when the task is complete.
async fn run_background_task(seconds: u64, value: Arc<Mutex<u64>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    let mut new_value = value.lock().await;
    println!("Shared value current: {}", new_value);
    *new_value = seconds;

    println!("Background task with duration of {} seconds has completed.", seconds);
    println!("Shared value has been updated to: {}", new_value);

    Ok(())
}

/// Runs multiple background tasks, specified by the durations in `times`.
/// Each task is represented by the `run_background_task` function.
async fn run_background_tasks(times: Vec<u64>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut futures = vec![];
    let value = Arc::new(Mutex::new(0));

    for seconds in times {
        futures.push(tokio::spawn(run_background_task(seconds, value.clone())));
    }

    futures_util::future::try_join_all(futures).await?;
    println!("All background tasks have completed.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let background_task_times = vec![5, 15, 20, 2];
    let mut sigterm = signal::unix::signal(SignalKind::terminate())?;

    println!("Process ID: {}", process::id());

    let background_task_handle = tokio::spawn(run_background_tasks(background_task_times));
    match sigterm.recv().await {
        Some(_) => {
            println!("Received signal to terminate. PID: {}", process::id());
            background_task_handle.await?.expect("fail to handle the tasks");
            println!("Process terminated.");
            Ok(())
        }
        None => Ok(()),
    }
}
