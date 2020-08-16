pub mod handlers;

use crate::storage::queue::Queue;
use crate::threadpool::ThreadPool;

pub struct Pipe<'q, PL> {
    topic: &'static str,
    queue: &'q Queue,
    handler: Box<dyn Fn(PL) + Sync + 'static>,
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
}

pub struct Center<'p, PL> {
    queue: &'p Queue,
    pipes: Vec<Pipe<'p, PL>>,
}

impl<'p, PL> Center<'p, PL>
where
    PL: serde::de::DeserializeOwned,
{
    pub fn new(pipes: Vec<Pipe<'p, PL>>, q: &'p Queue) -> &'p Self {
        for pipe in pipes.iter() {
            let pool = ThreadPool::new(5);
            loop {
                pool.execute(|| {
                    pipe.pop();
                });
            }
        }

        &Center {
            pipes: pipes,
            queue: q,
        }
    }
}
