use std::collections::BTreeSet;

pub type NumType = u32;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Range {
    min: NumType,
    max: NumType
}

impl Range {
    pub fn new(min: NumType, max: NumType) -> Range {
        Range { min: min, max: max }
    }
}

pub struct RangeSet {
    available_ranges: BTreeSet<Range>
}

impl RangeSet {
    pub fn new(initial_range: Range) -> RangeSet {
        let mut rs = RangeSet { available_ranges: BTreeSet::new() };
        rs.available_ranges.insert(initial_range);
        rs
    }

    pub fn take_any_one(&mut self) -> Option<NumType> {
        if let Some(range) = self.next_range() {
            self.available_ranges.remove(&range);
            if range.min == range.max {
                Some(range.min)
            } else {
                self.available_ranges.insert(Range::new(1 + range.min,
                                                        range.max));
                Some(range.min)
            }
        } else {
            None
        }
    }

    fn next_range(&self) -> Option<Range> {
        if let Some(&range) = self.available_ranges.iter().next() {
            Some(range)
        } else {
            None
        }
    }
}
