use super::update_strategy::{
    AgedBrieUpdate, BackstgeUpdate, DefaultUpdate, EmptyUpdate, UpdateStrategy,
};
use std::fmt::{self, Display};

pub struct ItemHolder {
    pub item: Item,
    update_strategy: Box<dyn UpdateStrategy>,
}

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl ItemHolder {
    pub fn update_item(&mut self) {
        self.update_strategy.update_item(&mut self.item);
    }
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }

    pub fn decrement_quality(&mut self) {
        self.quality = self.quality - 1;
    }

    pub fn increment_quality(&mut self) {
        self.quality = self.quality + 1;
    }

    pub fn decrement_sell_in(&mut self) {
        self.sell_in = self.sell_in - 1;
    }
}

#[non_exhaustive]
pub enum ItemType {
    DexterityVext,
    AgedBrie,
    ElixierOfTheMongoose,
    SulfurasHandOfRagnaros,
    BackstagePassesTAFKAL80ETCConcert,
    ConjuredManaCake,
}

pub struct ItemHolderFactory;
impl ItemHolderFactory {
    pub fn create_item_holder(item_type: ItemType, sell_in: i32, quality: i32) -> ItemHolder {
        match item_type {
            ItemType::DexterityVext => ItemHolder {
                item: Item::new("+5 Dexterity Vest", sell_in, quality),
                update_strategy: Box::new(DefaultUpdate),
            },
            ItemType::AgedBrie => ItemHolder {
                item: Item::new("Aged Brie", sell_in, quality),
                update_strategy: Box::new(AgedBrieUpdate),
            },
            ItemType::ElixierOfTheMongoose => ItemHolder {
                item: Item::new("Elixir of the Mongoose", sell_in, quality),
                update_strategy: Box::new(DefaultUpdate),
            },
            ItemType::SulfurasHandOfRagnaros => ItemHolder {
                item: Item::new("Sulfuras, Hand of Ragnaros", sell_in, quality),
                update_strategy: Box::new(EmptyUpdate),
            },
            ItemType::BackstagePassesTAFKAL80ETCConcert => ItemHolder {
                item: Item::new(
                    "Backstage passes to a TAFKAL80ETC concert",
                    sell_in,
                    quality,
                ),
                update_strategy: Box::new(BackstgeUpdate),
            },
            ItemType::ConjuredManaCake => ItemHolder {
                item: Item::new("Conjured Mana Cake", sell_in, quality),
                update_strategy: Box::new(DefaultUpdate),
            },
            _ => unimplemented!(),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub item_holders: Vec<ItemHolder>,
}

impl GildedRose {
    pub fn new(item_holders: Vec<ItemHolder>) -> GildedRose {
        GildedRose { item_holders }
    }

    pub fn update_quality(&mut self) {
        self.item_holders
            .iter_mut()
            .for_each(|item_holder| item_holder.update_item())
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item, ItemHolderFactory, ItemType};

    #[test]
    pub fn test_increment_quality() {
        let mut item = Item::new("asdf", 10, 20);

        item.increment_quality();

        assert_eq!(21, item.quality);
    }

    #[test]
    pub fn test_decrement_quality() {
        let mut item = Item::new("asdf", 10, 20);

        item.decrement_quality();

        assert_eq!(19, item.quality);
    }

    #[test]
    pub fn test_decrement_sell_in() {
        let mut item = Item::new("asdf", 10, 20);

        item.decrement_sell_in();

        assert_eq!(9, item.sell_in);
    }

    #[test]
    pub fn test_dexterity_vest() {
        let name = "+5 Dexterity Vest";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::DexterityVext,
            10,
            20,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(9, rose.item_holders[0].item.sell_in);
        assert_eq!(19, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn test_aged_bire() {
        let name = "Aged Brie";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::AgedBrie,
            2,
            0,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(1, rose.item_holders[0].item.sell_in);
        assert_eq!(1, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn test_elixier_of_the_mongoose() {
        let name = "Elixir of the Mongoose";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::ElixierOfTheMongoose,
            5,
            7,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(4, rose.item_holders[0].item.sell_in);
        assert_eq!(6, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn sulfuras_hand_of_ragnaros_v1() {
        let name = "Sulfuras, Hand of Ragnaros";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::SulfurasHandOfRagnaros,
            0,
            80,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(0, rose.item_holders[0].item.sell_in);
        assert_eq!(80, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn sulfuras_hand_of_ragnaros_v2() {
        let name = "Sulfuras, Hand of Ragnaros";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::SulfurasHandOfRagnaros,
            -1,
            80,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(-1, rose.item_holders[0].item.sell_in);
        assert_eq!(80, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn backstage_passes_v1() {
        let name = "Backstage passes to a TAFKAL80ETC concert";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::BackstagePassesTAFKAL80ETCConcert,
            15,
            20,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(14, rose.item_holders[0].item.sell_in);
        assert_eq!(21, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn backstage_passes_v2() {
        let name = "Backstage passes to a TAFKAL80ETC concert";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::BackstagePassesTAFKAL80ETCConcert,
            10,
            49,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(9, rose.item_holders[0].item.sell_in);
        assert_eq!(50, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn backstage_passes_v3() {
        let name = "Backstage passes to a TAFKAL80ETC concert";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::BackstagePassesTAFKAL80ETCConcert,
            5,
            49,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(4, rose.item_holders[0].item.sell_in);
        assert_eq!(50, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn conjured_mana_cake() {
        let name = "Conjured Mana Cake";
        let items = vec![ItemHolderFactory::create_item_holder(
            ItemType::ConjuredManaCake,
            3,
            6,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.item_holders[0].item.name);
        assert_eq!(2, rose.item_holders[0].item.sell_in);
        assert_eq!(5, rose.item_holders[0].item.quality);
    }

    #[test]
    pub fn multiple_updates() {
        let items = vec![
            ItemHolderFactory::create_item_holder(ItemType::DexterityVext, 10, 20),
            ItemHolderFactory::create_item_holder(ItemType::AgedBrie, 2, 0),
            ItemHolderFactory::create_item_holder(ItemType::ElixierOfTheMongoose, 5, 7),
            ItemHolderFactory::create_item_holder(ItemType::SulfurasHandOfRagnaros, 0, 80),
            ItemHolderFactory::create_item_holder(ItemType::SulfurasHandOfRagnaros, -1, 80),
            ItemHolderFactory::create_item_holder(
                ItemType::BackstagePassesTAFKAL80ETCConcert,
                15,
                20,
            ),
            ItemHolderFactory::create_item_holder(
                ItemType::BackstagePassesTAFKAL80ETCConcert,
                10,
                49,
            ),
            ItemHolderFactory::create_item_holder(
                ItemType::BackstagePassesTAFKAL80ETCConcert,
                5,
                49,
            ),
            ItemHolderFactory::create_item_holder(ItemType::ConjuredManaCake, 3, 6),
        ];
        let mut rose = GildedRose::new(items);
        for _ in 0..=29 {
            rose.update_quality();
        }

        assert_eq!(-20, rose.item_holders[0].item.sell_in);
        assert_eq!(0, rose.item_holders[0].item.quality);

        assert_eq!(-28, rose.item_holders[1].item.sell_in);
        assert_eq!(50, rose.item_holders[1].item.quality);

        assert_eq!(-25, rose.item_holders[2].item.sell_in);
        assert_eq!(0, rose.item_holders[2].item.quality);

        assert_eq!(0, rose.item_holders[3].item.sell_in);
        assert_eq!(80, rose.item_holders[3].item.quality);

        assert_eq!(-1, rose.item_holders[4].item.sell_in);
        assert_eq!(80, rose.item_holders[4].item.quality);

        assert_eq!(-15, rose.item_holders[5].item.sell_in);
        assert_eq!(0, rose.item_holders[5].item.quality);

        assert_eq!(-20, rose.item_holders[6].item.sell_in);
        assert_eq!(0, rose.item_holders[6].item.quality);

        assert_eq!(-25, rose.item_holders[7].item.sell_in);
        assert_eq!(0, rose.item_holders[7].item.quality);

        assert_eq!(-27, rose.item_holders[8].item.sell_in);
        assert_eq!(0, rose.item_holders[8].item.quality);
    }
}
