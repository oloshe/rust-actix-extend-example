use actix::{AsyncContext, Context, MailboxError};

use super::*;
pub trait ActixContextExtend {
    fn add_later(&mut self, add_num: u16, secs: u64) -> ();
}
impl ActixContextExtend for Context<MyActor> {
    fn add_later(&mut self, add_num: u16, secs: u64) -> () {
        println!("counter will add {}, after {} second(s)", add_num, secs);
        self.notify_later(CounterAdd(add_num), Duration::from_secs(secs));
    }
}

pub trait ActixMailboxSimplifyExtend<T> {
    fn handle_mailbox<F>(self, handle_fn: F)
    where
        F: FnOnce(T) -> ();
}

impl<T> ActixMailboxSimplifyExtend<T> for Result<T, MailboxError> {
    fn handle_mailbox<F>(self, handle_fn: F)
    where
        F: FnOnce(T) -> () {
        match self {
            Ok(data) => handle_fn(data),
            Err(e) => eprintln!("common handle MailboxError: {}", e),
        }
    }
}