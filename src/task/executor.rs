// Async executor and Task Scheduler.

use super::{Task, TaskId};
use alloc::{collections::BTreeMap, sync::Arc};
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use crossbeam_queue::ArrayQueue;

/// Cooperative Task Executor managing running and sleeping kernel tasks.
pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    /// Add a task to the executor.
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            panic!("Task with same ID already exists!");
        }
        self.task_queue.push(task_id).expect("Task queue full");
    }

    /// Run the executor loop until all tasks yield or halt.
    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    fn run_ready_tasks(&mut self) {
        while let Some(task_id) = self.task_queue.pop() {
            let task = match self.tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue, // Task no longer exists
            };

            let waker = self
                .waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, self.task_queue.clone()));
            let mut context = Context::from_waker(waker);

            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // Task completed; clean up resources
                    self.tasks.remove(&task_id);
                    self.waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }

    fn sleep_if_idle(&mut self) {
        x86_64::instructions::interrupts::disable();
        if self.task_queue.is_empty() {
            // Enable interrupts and sleep atomically via hlt
            x86_64::instructions::interrupts::enable_and_hlt();
        } else {
            x86_64::instructions::interrupts::enable();
        }
    }
}

/// Minimal Waker implementation to re-queue tasks on completion/notification.
struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        let waker = Arc::new(TaskWaker {
            task_id,
            task_queue,
        });
        let raw_waker = RawWaker::new(
            Arc::into_raw(waker) as *const (),
            &VTABLE,
        );
        unsafe { Waker::from_raw(raw_waker) }
    }

    fn wake_task(&self) {
        let _ = self.task_queue.push(self.task_id);
    }
}

const VTABLE: RawWakerVTable = RawWakerVTable::new(
    |ptr| {
        let waker = unsafe { Arc::from_raw(ptr as *const TaskWaker) };
        let cloned = waker.clone();
        core::mem::forget(waker);
        RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
    },
    |ptr| {
        let waker = unsafe { Arc::from_raw(ptr as *const TaskWaker) };
        waker.wake_task();
    },
    |ptr| {
        let waker = unsafe { Arc::from_raw(ptr as *const TaskWaker) };
        waker.wake_task();
    },
    |ptr| {
        drop(unsafe { Arc::from_raw(ptr as *const TaskWaker) });
    },
);