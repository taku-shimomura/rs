use rs::ds::queue::Queue;

fn main() {
    let mut q = Queue::default();
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    println!("{q:?}");
    println!("front: {:?}", q.front());
    println!("back: {:?}", q.back());
    q.dequeue();
    println!("{q:?}");
    println!("front: {:?}", q.front());
    println!("back: {:?}", q.back());
    q.dequeue();
    println!("{q:?}");
    println!("front: {:?}", q.front());
    println!("back: {:?}", q.back());
    q.dequeue();
    println!("front: {:?}", q.front());
    println!("back: {:?}", q.back());
}
