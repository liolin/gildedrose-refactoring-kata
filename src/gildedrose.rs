use std::fmt::{self, Display};

type FnPtr = fn(&mut Item) -> ();

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
    update_strategy: FnPtr,
}

impl Item {
    pub fn new(
        name: impl Into<String>,
        sell_in: i32,
        quality: i32,
        update_strategy: FnPtr,
    ) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
            update_strategy,
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

    pub fn update(&mut self) {
        (self.update_strategy)(self)
    }
}

pub struct ItemFactory;
impl ItemFactory {
    pub fn create_dexterity_vest(sell_in: i32, quality: i32) -> Item {
        Item::new("+5 Dexterity Vest", sell_in, quality, |item| {
            if item.quality > 0 {
                item.decrement_quality()
            }

            item.decrement_sell_in();

            if item.sell_in < 0 && item.quality > 0 {
                item.decrement_quality()
            }
        })
    }

    pub fn create_aged_brie(sell_in: i32, quality: i32) -> Item {
        Item::new("Aged Brie", sell_in, quality, |item| {
            if item.quality < 50 {
                item.increment_quality();
            }

            item.decrement_sell_in();

            if item.sell_in < 0 && item.quality < 50 {
                item.increment_quality();
            }
        })
    }

    pub fn create_elixier_of_the_mongoose(sell_in: i32, quality: i32) -> Item {
        Item::new("Elixir of the Mongoose", sell_in, quality, |item| {
            if item.quality > 0 {
                item.decrement_quality();
            }

            item.decrement_sell_in();

            if item.sell_in < 0 && item.quality > 0 {
                item.decrement_quality();
            }
        })
    }

    pub fn create_sulfuras_hand_of_ragnaros(sell_in: i32, quality: i32) -> Item {
        Item::new("Sulfuras, Hand of Ragnaros", sell_in, quality, |_| {})
    }

    pub fn create_backstage_passes_to_a_concert(sell_in: i32, quality: i32) -> Item {
        Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            sell_in,
            quality,
            |item| {
                if item.quality < 50 {
                    item.increment_quality();
                }

                if item.quality < 50 && item.sell_in < 11 {
                    item.increment_quality();
                }

                if item.quality < 50 && item.sell_in < 6 {
                    item.increment_quality();
                }
            },
        )
    }

    pub fn create_conjured_mana_cake(sell_in: i32, quality: i32) -> Item {
        Item::new("Conjured Mana Cake", sell_in, quality, |item| {
            if item.quality > 0 {
                item.decrement_quality();
            }

            item.decrement_sell_in();

            if item.sell_in < 0 && item.quality > 0 {
                item.decrement_quality();
            }
        })
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.decrement_quality();
                    }
                }
            } else {
                if item.quality < 50 {
                    item.increment_quality();

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.increment_quality();
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.increment_quality();
                            }
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.decrement_sell_in();
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                        if item.quality > 0 {
                            if item.name != "Sulfuras, Hand of Ragnaros" {
                                item.decrement_quality();
                            }
                        }
                    } else {
                        item.quality = item.quality - item.quality;
                    }
                } else {
                    if item.quality < 50 {
                        item.increment_quality();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item, ItemFactory};

    #[test]
    pub fn test_increment_quality() {
        let mut item = Item::new("asdf", 10, 20, |i| {});

        item.increment_quality();

        assert_eq!(21, item.quality);
    }

    #[test]
    pub fn test_decrement_quality() {
        let mut item = Item::new("asdf", 10, 20, |i| {});

        item.decrement_quality();

        assert_eq!(19, item.quality);
    }

    #[test]
    pub fn test_decrement_sell_in() {
        let mut item = Item::new("asdf", 10, 20, |i| {});

        item.decrement_sell_in();

        assert_eq!(9, item.sell_in);
    }

    #[test]
    pub fn test_dexterity_vest() {
        let name = "+5 Dexterity Vest";
        let items = vec![Item::new(name, 10, 20, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(19, rose.items[0].quality);
    }

    #[test]
    pub fn test_aged_bire() {
        let name = "Aged Brie";
        let items = vec![Item::new(name, 2, 0, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(1, rose.items[0].sell_in);
        assert_eq!(1, rose.items[0].quality);
    }

    #[test]
    pub fn test_elixier_of_the_mongoose() {
        let name = "Elixir of the Mongoose";
        let items = vec![Item::new(name, 5, 7, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(4, rose.items[0].sell_in);
        assert_eq!(6, rose.items[0].quality);
    }

    #[test]
    pub fn sulfuras_hand_of_ragnaros_v1() {
        let name = "Sulfuras, Hand of Ragnaros";
        let items = vec![Item::new(name, 0, 80, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(0, rose.items[0].sell_in);
        assert_eq!(80, rose.items[0].quality);
    }

    #[test]
    pub fn sulfuras_hand_of_ragnaros_v2() {
        let name = "Sulfuras, Hand of Ragnaros";
        let items = vec![Item::new(name, -1, 80, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(-1, rose.items[0].sell_in);
        assert_eq!(80, rose.items[0].quality);
    }

    #[test]
    pub fn backstage_passes_v1() {
        let name = "Backstage passes to a TAFKAL80ETC concert";
        let items = vec![Item::new(name, 15, 20, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(14, rose.items[0].sell_in);
        assert_eq!(21, rose.items[0].quality);
    }

    #[test]
    pub fn backstage_passes_v2() {
        let name = "Backstage passes to a TAFKAL80ETC concert";
        let items = vec![Item::new(name, 10, 49, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(50, rose.items[0].quality);
    }

    #[test]
    pub fn backstage_passes_v3() {
        let name = "Backstage passes to a TAFKAL80ETC concert";
        let items = vec![Item::new(name, 5, 49, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(4, rose.items[0].sell_in);
        assert_eq!(50, rose.items[0].quality);
    }

    #[test]
    pub fn conjured_mana_cake() {
        let name = "Conjured Mana Cake";
        let items = vec![Item::new(name, 3, 6, |i| {})];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(name, rose.items[0].name);
        assert_eq!(2, rose.items[0].sell_in);
        assert_eq!(5, rose.items[0].quality);
    }
}
