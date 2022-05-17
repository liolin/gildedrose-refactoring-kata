mod gildedrose;

use gildedrose::{GildedRose, Item};

fn main() {
    let items = vec![
        Item::new("+5 Dexterity Vest", 10, 20, |i| {}),
        Item::new("Aged Brie", 2, 0, |i| {}),
        Item::new("Elixir of the Mongoose", 5, 7, |i| {}),
        Item::new("Sulfuras, Hand of Ragnaros", 0, 80, |i| {}),
        Item::new("Sulfuras, Hand of Ragnaros", -1, 80, |i| {}),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20, |i| {}),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 49, |i| {}),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49, |i| {}),
        // this conjured item does not work properly yet
        Item::new("Conjured Mana Cake", 3, 6, |i| {}),
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
