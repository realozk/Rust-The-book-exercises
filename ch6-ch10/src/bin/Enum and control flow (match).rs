#[derive(Debug)]
enum Item {
    Sword(i32),
    Potion(i32),
    Map(bool),
}

struct InventorySlot {
    index: u8,
    contents: Option<Item>,
}

impl InventorySlot {
    fn use_item(&self) {
        if let Some(item) = &self.contents {
            println!("You used the item in slot {}:", self.index);

            match item {
                Item::Sword(damage) => {
                    println!(" You dealt {} damage to the enemy.", damage);
                }
                Item::Potion(heal_amount) => {
                    println!("You recovered {} health points.", heal_amount);
                }
                Item::Map(is_discovered) => {
                    if *is_discovered {
                        println!("Looking at the map... You know this place!");
                    } else {
                        println!("You opened a NEW map. Area unknown.");
                    }
                }
            }
        } else {
            println!("Slot {} is empty! Nothing to use.", self.index);
        }
    }
}

fn main() {
    let slot1 = InventorySlot {
        index: 1,
        contents: Some(Item::Sword(50)),
    };

    let slot2 = InventorySlot {
        index: 2,
        contents: None,
    };

    let slot3 = InventorySlot {
        index: 3,
        contents: Some(Item::Potion(100)),
    };
    let slot4: InventorySlot = InventorySlot {
        index: 4,
        contents: Some(Item::Map((true))),
    };

    println!("--- Turn 1 ---");
    slot1.use_item();

    println!("\n--- Turn 2 ---");
    slot2.use_item();

    println!("\n--- Turn 3 ---");
    slot3.use_item();

    println!("\n--- Turn 4 ---");
    slot4.use_item();
}