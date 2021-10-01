use std::time::Duration;

use actix_extend::{
    CounterAdd, DoubleAfterDelta, GetCounter, MyActor, ShutDown, 
    actix::{self, Actor, clock::sleep},
    extend::ActixMailboxSimplifyExtend}; // 引入拓展特性

fn main() {
    let run_ret = actix::run(async move {
        let actor = MyActor::default();
        let addr = actor.start();
        println!("=[case 1]=========================");
        let current_value = addr.send(GetCounter).await.unwrap();
        println!("init value is: {}", current_value);
        let fut = addr.send(DoubleAfterDelta {
            secs: 1,
        });

        // add during DoubleAfterDelta's Handler waiting
        sleep(Duration::from_millis(200)).await;
        addr.do_send(CounterAdd(3));

        sleep(Duration::from_millis(200)).await;
        addr.do_send(CounterAdd(5)); 

        let _ = fut.await; // wait a seconds.

        let current_value = addr.send(GetCounter).await.unwrap();
        println!("value is: {}", current_value);
        sleep(Duration::from_secs(2)).await;
        
        let current_value = addr.send(GetCounter).await.unwrap();
        println!("value is: {}", current_value);

        println!("=[case 2]=========================");
        addr.do_send(ShutDown);
        let ret = addr.send(GetCounter).await;
        // use the added method in ActixMailboxSimplifyExtend
        ret.handle_mailbox(|_| {
            unreachable!("unpossible to reach here due to MailboxError must be encountered.");
        });
    });
    println!("actix-run: {:?}", run_ret);
}
