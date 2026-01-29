use std::collections::{HashMap, btree_map::Keys};

fn main (){

    //creat hashmap
    let mut scor = HashMap::new();
    scor.insert(String::from("adnan"), 4);
    scor.insert(String::from("omar"), 702);

    //get a valude from the name
    let adnan = String::from("adnan");

    let sc = scor.get(&adnan).copied().unwrap_or(0);

    println!("{}",sc);


    //get the key and the value
    for(Keys,value) in &scor{
        println!("{}: {}" , Keys , value);
    }
    scor.entry(String::from("meshy")).or_insert(50);
    println!("{:?}", scor);


    //count the repeted word
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}