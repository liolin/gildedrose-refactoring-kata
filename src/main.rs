mod gildedrose;

use gildedrose::{EmptyUpdate, GildedRose, Item};

fn main() {
    let items = vec![
        Item::new("+5 Dexterity Vest", 10, 20, Box::new(EmptyUpdate)),
        Item::new("Aged Brie", 2, 0, Box::new(EmptyUpdate)),
        Item::new("Elixir of the Mongoose", 5, 7, Box::new(EmptyUpdate)),
        Item::new("Sulfuras, Hand of Ragnaros", 0, 80, Box::new(EmptyUpdate)),
        Item::new("Sulfuras, Hand of Ragnaros", -1, 80, Box::new(EmptyUpdate)),
        Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            15,
            20,
            Box::new(EmptyUpdate),
        ),
        Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            10,
            49,
            Box::new(EmptyUpdate),
        ),
        Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            5,
            49,
            Box::new(EmptyUpdate),
        ),
        // this conjured item does not work properly yet
        Item::new("Conjured Mana Cake", 3, 6, Box::new(EmptyUpdate)),
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
