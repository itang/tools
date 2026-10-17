//! Bounded-concurrency FIFO task queue.

use std::future::Future;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Runs at most `concurrency` tasks at once; tasks wait their turn in FIFO
/// order (tokio's semaphore is fair).
#[derive(Clone)]
pub struct TaskQueue {
    semaphore: Arc<Semaphore>,
    concurrency: usize,
}

impl TaskQueue {
    pub fn new(concurrency: usize) -> Self {
        let concurrency = concurrency.max(1);
        Self {
            semaphore: Arc::new(Semaphore::new(concurrency)),
            concurrency,
        }
    }

    /// Total configured concurrency.
    #[allow(dead_code)]
    pub fn concurrency(&self) -> usize {
        self.concurrency
    }

    /// Enqueue a task, holding a permit for its whole duration.
    pub async fn add<F, Fut, T>(&self, task: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        let permit: OwnedSemaphorePermit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("task queue semaphore closed");
        let result = task().await;
        drop(permit);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn runs_tasks_fifo_and_limits_concurrency() {
        let queue = TaskQueue::new(1);
        let log = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        for value in 0..3 {
            let queue = queue.clone();
            let log = log.clone();
            handles.push(tokio::spawn(async move {
                queue
                    .add(|| async move {
                        log.lock().unwrap().push(value);
                    })
                    .await;
            }));
        }
        for handle in handles {
            handle.await.unwrap();
        }
        assert_eq!(*log.lock().unwrap(), vec![0, 1, 2]);
    }

    #[tokio::test]
    async fn never_exceeds_concurrency_limit() {
        let queue = TaskQueue::new(2);
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..6 {
            let queue = queue.clone();
            let active = active.clone();
            let peak = peak.clone();
            handles.push(tokio::spawn(async move {
                queue
                    .add(|| async move {
                        let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                        peak.fetch_max(now, Ordering::SeqCst);
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        active.fetch_sub(1, Ordering::SeqCst);
                    })
                    .await;
            }));
        }
        for handle in handles {
            handle.await.unwrap();
        }
        assert!(peak.load(Ordering::SeqCst) <= 2);
    }
}
