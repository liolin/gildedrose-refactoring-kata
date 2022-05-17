mod gildedrose;

use gildedrose::{GildedRose, ItemFactory};

fn main() {
    let items = vec![
        ItemFactory::create_dexterity_vest(10, 20),
        ItemFactory::create_aged_brie(2, 0),
        ItemFactory::create_elixier_of_the_mongoose(5, 7),
        ItemFactory::create_sulfuras_hand_of_ragnaros(0, 80),
        ItemFactory::create_sulfuras_hand_of_ragnaros(-1, 80),
        ItemFactory::create_backstage_passes_to_a_concert(15, 20),
        ItemFactory::create_backstage_passes_to_a_concert(10, 49),
        ItemFactory::create_backstage_passes_to_a_concert(5, 49),
        // this conjured item does not work properly yet
        ItemFactory::create_conjured_mana_cake(3, 6),
    ];
    let mut rose = GildedRose::new(items);

    println!("OMGHAI!");
    for i in 0..=30 {
        println!("-------- day {} --------", i);
        println!("name, sellIn, quality");
        for item in &rose.items {
            println!("{}", item);
        }
        println!();
        rose.update_quality();
    }
}
