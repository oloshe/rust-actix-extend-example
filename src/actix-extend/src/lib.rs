use std::{ops::AddAssign, time::Duration};

use actix::{Actor, ActorContext, ActorFutureExt, AsyncContext, Context, Handler, Message, ResponseActFuture, WrapFuture};

pub mod extend;
pub use actix;
use extend::{ActixContextExtend, ActixMailboxSimplifyExtend};

pub struct MyActor {
    pub counter: u16,
}

impl Default for MyActor {
    fn default() -> Self {
        Self { counter: Default::default() }
    }
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

/// counter add message
#[derive(Debug, Message)]
#[rtype("u16")]
pub struct GetCounter;

impl Handler<GetCounter> for MyActor {
    type Result = u16;

    fn handle(&mut self, _: GetCounter, _: &mut Self::Context) -> Self::Result {
        self.counter
    }
}


/// counter add message
#[derive(Debug, Message)]
#[rtype("u16")]
pub struct CounterAdd(pub u16);

impl Handler<CounterAdd> for MyActor {
    type Result = u16;

    fn handle(&mut self, msg: CounterAdd, _: &mut Self::Context) -> Self::Result {
        println!("add {}", msg.0);
        self.counter.add_assign(msg.0);
        self.counter
    }
}

/// get counter's value change during the [`Duration`]
#[derive(Debug, Message)]
#[rtype("u16")]
pub struct GetDelta(pub Duration);

impl Handler<GetDelta> for MyActor {
    type Result = ResponseActFuture<Self, u16>;

    fn handle(&mut self, msg: GetDelta, _: &mut Self::Context) -> Self::Result {
        let init_value = self.counter;
        Box::pin(
            async move {
                actix::clock::sleep(msg.0).await;
            }
                .into_actor(self)
                .map(move |_, actor, _| {
                    actor.counter - init_value
                })
        )
    }
}

/// an odd mission required by the lovely PM
#[derive(Debug, Message)]
#[rtype("()")]
pub struct DoubleAfterDelta {
    pub secs: u64
}

impl Handler<DoubleAfterDelta> for MyActor {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: DoubleAfterDelta, ctx: &mut Self::Context) -> Self::Result {
        Box::pin({
            let addr = ctx.address();
            addr.send(GetDelta(
                Duration::from_secs(msg.secs)
            ))
                .into_actor(self)
                .map(move |ret, _, ctx| {
                    ret.handle_mailbox(|delta| {
                        ctx.add_later(delta, msg.secs);
                    });
                })
        })
    }
}

/// shutdown
#[derive(Debug, Message)]
#[rtype("()")]
pub struct ShutDown;

impl Handler<ShutDown> for MyActor {
    type Result = ();

    fn handle(&mut self, _: ShutDown, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let _ = actix::run(async move {
            let addr = MyActor::start_default();
            let _ = addr.send(CounterAdd(6)).await;
            let ret = addr.send(CounterAdd(3)).await;
            assert_eq!(ret.unwrap(), 9);
            actix::System::current().stop();
        });
    }

    #[test]
    fn test_duration() {
        let _ = actix::run(async move {
            let addr = MyActor::start_default();
            let _ = addr.send(CounterAdd(2)).await;
            let fut = addr.send(GetDelta(Duration::from_secs(2)));
            let _ = addr.send(CounterAdd(4)).await;
            let ret = addr.send(CounterAdd(12)).await.unwrap();
            assert_eq!(ret, 18);
            println!("counter is {}", ret);
            println!("waitting duration...");
            let ret = fut.await.unwrap();
            assert_eq!(ret, 16);
            println!("delta during sleep is {}", ret);
        });
    }
}