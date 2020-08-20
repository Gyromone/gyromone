pub mod handlers;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::storage::queue::Queue;
use crate::threadpool::ThreadPool;

const PIPE_THREAD_SIZE: usize = 5;

type PipeHandler<PL> = Arc<Box<dyn Fn(PL) + Send + Sync + 'static>>;

pub struct Pipe<PL: 'static> {
    topic: &'static str,
    queue: Arc<Queue>,
    handler: PipeHandler<PL>,
}

impl<PL> Pipe<PL>
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

    pub fn subscribe(&self) {
        let pool = ThreadPool::new(PIPE_THREAD_SIZE);

        let queue = Arc::clone(&self.queue);
        let topic = self.topic.clone();
        let handler = Arc::clone(&self.handler);
        pool.execute(move || loop {
            let v: String = match queue.pop(topic) {
                None => return (),
                Some(v) => v,
            };
            let payload: PL = serde_json::from_slice(v.as_bytes()).unwrap();

            (*handler)(payload);

            println!("subscribe");
            thread::sleep(Duration::from_millis(200));
        });
    }

    pub fn new(topic: &'static str, queue: Arc<Queue>, handler: PipeHandler<PL>) -> Self {
        Pipe {
            topic: topic,
            queue: queue,
            handler: handler,
        }
    }
}

pub struct Center<'pipe, PL: 'static> {
    pipes: Vec<&'pipe Pipe<PL>>,
}

impl<'pipe, PL> Center<'pipe, PL>
where
    PL: serde::de::DeserializeOwned,
{
    pub fn new(pipes: Vec<&'pipe Pipe<PL>>) -> Self {
        for pipe in pipes.iter() {
            pipe.subscribe()
        }

        Center { pipes: pipes }
    }
}
