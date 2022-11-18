pub mod linkedqueue;
use crate::linkedqueue::LinkedQueue;
fn main() {

    let queue = LinkedQueue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);
 
    println!("items in queue are {} {} {} {}", queue.pop().unwrap(),
       queue.pop().unwrap(), queue.pop().unwrap(), queue.pop().unwrap());  
}
