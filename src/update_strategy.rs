use super::gildedrose::Item;
pub trait UpdateStrategy {
    fn update_item(&self, item: &mut Item);
}

pub struct EmptyUpdate;
impl UpdateStrategy for EmptyUpdate {
    fn update_item(&self, _item: &mut Item) {
        // Empty on purpose
    }
}

pub struct DefaultUpdate;
impl UpdateStrategy for DefaultUpdate {
    fn update_item(&self, item: &mut Item) {
        if item.quality > 0 {
            item.decrement_quality()
        }

        item.decrement_sell_in();

        if item.sell_in < 0 && item.quality > 0 {
            item.decrement_quality()
        }
    }
}

pub struct AgedBrieUpdate;
impl UpdateStrategy for AgedBrieUpdate {
    fn update_item(&self, item: &mut Item) {
        if item.quality < 50 {
            item.increment_quality();
        }

        item.decrement_sell_in();

        if item.sell_in < 0 && item.quality < 50 {
            item.increment_quality();
        }
    }
}

pub struct BackstgeUpdate;
impl UpdateStrategy for BackstgeUpdate {
    fn update_item(&self, item: &mut Item) {
        if item.quality < 50 {
            item.increment_quality();
        }

        if item.quality < 50 && item.sell_in < 11 {
            item.increment_quality();
        }

        if item.quality < 50 && item.sell_in < 6 {
            item.increment_quality();
        }

        item.decrement_sell_in();

        if item.sell_in < 0 {
            item.quality = 0;
        }
    }
}
