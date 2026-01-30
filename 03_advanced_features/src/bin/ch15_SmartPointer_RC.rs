//we have a malware signature and its a virible that we need to read and use muitlbe times throw scanners, how?
//by using the RC pointer we can
use std::rc::Rc;

fn main() {
    let virus_signature = Rc::new(String::from("ana_virus_signature_X86_64"));

    let scanner_a = Rc::clone(&virus_signature);
    println!("Scanner analyzing using {:?}", &scanner_a);
    println!("Current Owners: {}", Rc::strong_count(&virus_signature)); //first use

    {
        let scanner_2 = Rc::clone(&virus_signature);
        println!("Scanner 2 analyzing using: {}", scanner_2);
        println!("Current Owners: {}", Rc::strong_count(&virus_signature)); // second use
    } //out of scop and still reading

    println!("Final Owners: {}", Rc::strong_count(&virus_signature)); // 2
}