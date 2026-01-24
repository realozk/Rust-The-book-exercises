
struct Human;
struct Robot;

pub trait Speak {
    fn say_hello(&self) -> String;
}

impl Speak for Human {
    fn say_hello(&self) -> String {
        String::from("salam")
    }
}

impl Speak for Robot {
    fn say_hello(&self) -> String {
        String::from("Beep")
    }
}

fn main (){
    let a = Human;
    let b = Robot;

    println!("Human says: {}", a.say_hello());
    println!("Robot says: {}", b.say_hello());

}

/*

struct Human;
struct Robot;

fn introduce(item:&impl  Speak){

    println!("Please welcome: {}", item.say_hello());

}

fn compare_speakers (){}

pub trait Speak {
    fn say_hello(&self) -> String;

   fn say_goodbye(&self) -> String {
        String::from("(Silent wave...)")
    }
}

impl Speak for Human {
fn say_hello(&self) -> String {
    String::from("salam")
}
fn say_goodbye(&self) -> String {
    String::from("bye")
}
}

impl Speak for Robot {
fn say_hello(&self) -> String {
    String::from("Beep")
}
}

fn main (){
    let a = Human;
    let b = Robot;

println!("Human says: {}", a.say_hello());
println!("Robot says: {}", b.say_hello());

introduce(&a);
introduce(&b);

}
 */