use crate::quotes::Quotes;
use crate::trade::Order;
use crate::LogEvent;
use crossbeam::channel::{bounded, Receiver, Sender};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

const QUOTES_THREAD: usize = 10;
const ORDERS_THREAD: usize = 5;
const LOG_THREAD: usize = 1;

lazy_static! {
    //行情总线
    static ref QUOTES:EventBus<Vec<Quotes>> = EventBus::new(QUOTES_THREAD);
    //交易总线
    static ref ORDERS: EventBus<Vec<Order>> = EventBus::new(ORDERS_THREAD);
    //日志事件总线
    static ref LOG: EventBus<LogEvent> = EventBus::new(LOG_THREAD);
}

pub fn start() {
    QUOTES.run();
    ORDERS.run();
    LOG.run();
}

pub fn consumer_quote<F>(f: F)
where
    F: FnMut(Vec<Quotes>) + Send + 'static,
{
    QUOTES.consumer(f)
}

pub fn send_quote(quote: Vec<Quotes>) {
    QUOTES.send(quote)
}

pub fn consumer_order<F>(f: F)
where
    F: FnMut(Vec<Order>) + Send + 'static,
{
    ORDERS.consumer(f)
}

pub fn send_order(order: Vec<Order>) {
    ORDERS.send(order)
}

pub fn consumer_log<F>(f: F)
where
    F: FnMut(LogEvent) + Send + 'static + Sized,
{
    LOG.consumer(f)
}

pub fn send_log(err: LogEvent) {
    LOG.send(err)
}

pub struct EventBus<T> {
    cb: Arc<Mutex<Vec<Box<dyn FnMut(T) + Send + 'static>>>>,
    tx: Sender<T>,
    rx: Receiver<T>,
    workers: usize,
}

impl<T: Send + Sync + 'static + Sized + Clone> EventBus<T> {
    pub fn new(workers: usize) -> Self {
        let (tx, rx) = bounded::<T>(10);
        let cb = Arc::new(Mutex::new(vec![]));
        Self {
            cb,
            tx,
            rx,
            workers,
        }
    }
    pub fn consumer<F>(&self, f: F)
    where
        F: FnMut(T) + Send + 'static,
    {
        if let Ok(mut guard) = self.cb.lock() {
            guard.push(Box::new(f));
        }
    }

    pub fn send(&self, msg: T) {
        let _ = self.tx.try_send(msg);
        //self.tx.send(msg).ok();
    }

    pub fn run(&self) {
        (0..self.workers).for_each(|_| {
            let rx = self.rx.clone();
            let cb = self.cb.clone();
            std::thread::spawn(move || {
                while let Ok(msg) = rx.recv() {
                    if let Ok(mut guard) = cb.lock() {
                        guard.iter_mut().for_each(|f| {
                            (f)(msg.clone());
                        });
                    }
                }
            });
        });
    }
}
