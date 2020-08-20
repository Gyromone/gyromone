pub mod handlers;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use crate::storage::queue::Queue;
use crate::threadpool::ThreadPool;

const PIPE_THREAD_SIZE: usize = 5;

type PipeHandler<PL> = Box<dyn Fn(PL) + Sync + 'static>;

pub struct Pipe<'q, PL> {
    topic: &'static str,
    queue: &'q Queue,
    handler: PipeHandler<PL>,
}

impl<'q, PL> Pipe<'q, PL>
where
    PL: serde::de::DeserializeOwned,
{
    pub fn push<T>(&self, value: T)
    where
        T: serde::Serialize,
    {
        let v_string = serde_json::to_string(&value).unwrap();
        self.queue.push(self.topic, v_string);
    }

    pub fn pop(&self) {
        let v: String = self.queue.pop(self.topic).unwrap();
        let payload: PL = serde_json::from_slice(v.as_bytes()).unwrap();

        (*self.handler)(payload)
    }

    pub fn subscribe(&self) {
        let pool = ThreadPool::new(PIPE_THREAD_SIZE);

        for i in 0..PIPE_THREAD_SIZE {
            pool.execute(|| loop {
                //self.pop();
                println!("execute");
                thread::sleep(Duration::from_millis(200));
            });
        }
    }

    pub fn new(topic: &'static str, queue: &'q Queue, handler: PipeHandler<PL>) -> Self {
        Pipe {
            topic: topic,
            queue: queue,
            handler: handler,
        }
    }
}

pub struct Center<'p, PL> {
    queue: &'p Queue,
    pipes: Vec<Pipe<'p, PL>>,
}

impl<'p, PL> Center<'p, PL>
where
    PL: serde::de::DeserializeOwned,
{
    pub fn new(pipes: Vec<Pipe<'p, PL>>, q: &'p Queue) -> Self {
        for pipe in pipes.iter() {
            pipe.subscribe()
        }

        Center {
            pipes: pipes,
            queue: q,
        }
    }
}
