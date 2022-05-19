use rust::gildedrose::{GildedRose, ItemHolderFactory, ItemType};

fn main() {
    let item_holders = vec![
        ItemHolderFactory::create_item_holder(ItemType::DexterityVext, 10, 20),
        ItemHolderFactory::create_item_holder(ItemType::AgedBrie, 2, 0),
        ItemHolderFactory::create_item_holder(ItemType::ElixierOfTheMongoose, 5, 7),
        ItemHolderFactory::create_item_holder(ItemType::SulfurasHandOfRagnaros, 0, 80),
        ItemHolderFactory::create_item_holder(ItemType::SulfurasHandOfRagnaros, -1, 80),
        ItemHolderFactory::create_item_holder(ItemType::BackstagePassesTAFKAL80ETCConcert, 15, 20),
        ItemHolderFactory::create_item_holder(ItemType::BackstagePassesTAFKAL80ETCConcert, 10, 49),
        ItemHolderFactory::create_item_holder(ItemType::BackstagePassesTAFKAL80ETCConcert, 5, 49),
        ItemHolderFactory::create_item_holder(ItemType::ConjuredManaCake, 3, 6),
    ];
    let mut rose = GildedRose::new(item_holders);

    println!("OMGHAI!");
    for i in 0..=30 {
        println!("-------- day {} --------", i);
        println!("name, sellIn, quality");
        for item_holder in &rose.item_holders {
            println!("{}", item_holder.item);
        }
        println!();
        rose.update_quality();
    }
}
