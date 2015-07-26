use std::collections::BTreeSet;

type NumType = u32;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    min: NumType,
    max: NumType
}

impl Range {
    fn new(min: NumType, max: NumType) -> Range {
        Range { min: min, max: max }
    }
}

struct RangeSet {
    available_ranges: BTreeSet<Range>
}

impl RangeSet {
    fn new(initial_range: Range) -> RangeSet {
        let mut rs = RangeSet { available_ranges: BTreeSet::new() };
        rs.available_ranges.insert(initial_range);
        rs
    }

    fn next_range(&self) -> Option<Range> {
        if let Some(&range) = self.available_ranges.iter().next() {
            Some(range)
        } else {
            None
        }
    }

    fn take_any_one(&mut self) -> Option<NumType> {
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
}


// eating a bagel
// fake tooth fuckin fell right out
// worst fucking dentist
