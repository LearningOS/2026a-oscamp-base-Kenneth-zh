//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // TODO: Create Arc<Mutex<usize>> with initial value 0
    let res = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    // TODO: Spawn n_threads threads
    for _ in 0..n_threads {
        let res = res.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..count_per_thread {
                *res.lock().unwrap() += 1
            }
        }));
    }
    // TODO: In each thread, lock() and increment count_per_thread times
    // TODO: Join all threads, return final value

    for handle in handles {
        handle.join().unwrap();
    }
    let result = *res.lock().unwrap();
    result
}

/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: Create Arc<Mutex<Vec<usize>>>
    // TODO: Each thread pushes its own id
    // TODO: After joining all threads, sort the result and return

    let result = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();

    for id in 0..n_threads {
        let result = result.clone();
        handles.push(thread::spawn(move || result.lock().unwrap().push(id)));
    }
    for handle in handles {
        let _ = handle.join();
    }
    let mut values = std::mem::take(&mut *result.lock().unwrap());
    values.sort();
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
