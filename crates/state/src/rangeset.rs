use std::ptr;
use wasm_bindgen::prelude::wasm_bindgen;
// use crate::change::MapMode;

pub trait AbstractRangeValue {
    fn get_start_side() -> isize {
        return 0;
    }

    fn get_end_side() -> isize {
        return 0;
    }

    // fn get_map_mode() -> MapMode {
    //     return MapMode::TrackDel;
    // }

    fn is_point() -> bool {
        return false;
    }

    fn eq(&self, other: &Self) -> bool {
        return ptr::eq(self, other);
    }

    fn create_range(&self, from: isize, to: Option<isize>) -> Range {
        let to = to.unwrap_or(from);

        return Range::create(from, to, self);
    }
}

pub trait RangeComparator<T> {
    fn compare_range(&self, from: isize, to: isize, active_a: Vec<T>, active_b: Vec<T>);

    fn compare_point(&self, from: isize, to: isize, point_a: Option<T>, point_b: Option<T>);
}

pub trait SpanIterator<T> {
    fn span(&self, from: isize, to: isize, active: Vec<T>, open_start: isize);

    fn point(&self, from: isize, to: isize, active: Vec<T>, open_start: isize, index: isize);
}

#[wasm_bindgen]
pub struct RangeValue;

impl AbstractRangeValue for RangeValue {}

struct Range {
    from: isize,

    to: isize,

    value: dyn AbstractRangeValue,
}

impl Range {
    fn create(from: isize, to: isize, value: AbstractRangeValue) -> Range {
        return Range { from, to, value };
    }
}

// fn compare_range(a: Range<impl AbstractRangeValue>, b: Range<impl AbstractRangeValue>) -> bool {
//     return a.from - b.from || a.value as isize - b.value.start_side as isize;
// }

pub enum C {}
