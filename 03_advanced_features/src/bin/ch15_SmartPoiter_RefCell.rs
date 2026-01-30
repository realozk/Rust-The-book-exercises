//i have HoneyPot and i want immutable struct that count the attacks but we cant change the immutable >:
//haha we can by using RefCell <:
use std::cell::RefCell;
use std::rc::Rc;

struct HoneyPot{
    ip: String,
    attack_counter: RefCell<u32>,
}
fn main() {
    let trap = HoneyPot{
        ip: String::from("10.10.10.702"),
        attack_counter: RefCell::new(0),
    };
    //see we didnt use & with trap and we will rdit it

    *trap.attack_counter.borrow_mut() += 1;
    *trap.attack_counter.borrow_mut() += 1;
    
    println!("Attacks detected on {}: {}", trap.ip, trap.attack_counter.borrow());

}
