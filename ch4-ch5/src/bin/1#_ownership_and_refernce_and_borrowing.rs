fn main() {
let mut data = String::from("oneisbetter");

    let size = read(&data);
    println!("the size {}", size);

 encrypto(&mut data);
println!("the encrypto is {}",data);

    delete(data);

    println!("all done");
}

fn read (s: &String) -> usize  {
    println!("loading...");
    s.len()
}

fn encrypto(s: &mut String){
println!("encrypting...");
    s.clear();
    s.push_str("fuckthisbetter");
}

fn delete(s:String) {
    println!("deleting...");


    println!("data is dropped");
}