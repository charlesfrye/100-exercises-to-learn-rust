use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    yield_now().await;
    let non_send = Rc::new(1);
    println!("{}", non_send);
}
