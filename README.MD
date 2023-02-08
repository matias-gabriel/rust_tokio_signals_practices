# Background Tasks in Rust with Tokio

This code demonstrates how to run multiple background tasks in Rust using the Tokio library. The code uses async/await syntax and Tokio's `tokio::spawn` function to run tasks concurrently. 

A vector of durations, `background_task_times`, is passed to the `run_background_tasks` function, which then creates and spawns a new task for each duration. Each task runs for its specified duration and increments a shared value. The shared value is stored in an `Arc<Mutex<u64>>`, which allows for safe, concurrent access to the value.

The main function listens for a `SIGTERM` signal using the `tokio::signal` library, and if it receives the signal, it waits for all the background tasks to complete before terminating the process.


