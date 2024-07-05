use signal::Signal;

fn main() {
    let mut signal = Signal::<i32>::new();

    let _id1 = signal.subscribe(|value| println!("Subscriber 1 received: {}", value));
    let id2 = signal.subscribe(|value| println!("Subscriber 2 received: {}", value));

    signal.call(&10);
    signal.unsubscribe(id2);
    signal.call(&20);
    signal.cleanup();

    signal.call(&30);
    signal.subscribe(|value| println!("New subscriber received: {}", value));
    signal.call(&40);
}