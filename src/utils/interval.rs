use std::{fmt, ops};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortedDisjointIntervalList<D>
where
    D: Clone,
{
    pub intervals: Vec<Interval<D>>,
}

impl<D> SortedDisjointIntervalList<D>
where
    D: Clone,
{
    pub fn new(mut intervals: Vec<Interval<D>>) -> Self {
        intervals.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        Self { intervals }
    }

    pub fn num_ranges(&self) -> usize {
        self.intervals.len()
    }

    pub fn span(&self) -> u64 {
        let mut span = 0;
        for interval in &self.intervals {
            span += interval.range.size_hint().0;
        }
        span as u64
    }

    pub fn intersect(&self, other: &Self) -> SortedDisjointIntervalList<D> {
        let mut ai: usize = 0;
        let mut bi: usize = 0;

        let mut intersections = Vec::new();
        while ai < self.intervals.len() && bi < other.intervals.len() {
            let a = &self.intervals[ai];
            let b = &other.intervals[bi];

            if let Some(intersection) = a.intersect(b) {
                intersections.push(intersection);
            }

            if a.range.end < b.range.end {
                ai += 1;
            } else {
                bi += 1;
            }
        }

        SortedDisjointIntervalList::new(intersections)
    }

    pub fn subtract(&self, other: &Self) -> SortedDisjointIntervalList<D> {
        let mut subtractions = Vec::new();
        for a in &self.intervals {
            let mut subtraction: Vec<Interval<D>> = vec![a.clone()];
            for b in &other.intervals {
                if let Some(a_comp) = subtraction.last() {
                    subtraction = a_comp.subtract(b)
                }
            }
            subtractions.extend(subtraction);
        }
        SortedDisjointIntervalList::new(subtractions)
    }

    pub fn merge(&self, other: &Self) -> SortedDisjointIntervalList<D> {
        let projection_intersection = self.intersect(other);
        let sub_self = self.subtract(&projection_intersection);
        let sub_other = other.subtract(&projection_intersection);

        let res = SortedDisjointIntervalList::new(
            sub_self
                .intervals
                .iter()
                .chain(sub_other.intervals.iter())
                .chain(projection_intersection.intervals.iter())
                .cloned()
                .collect(),
        );

        res
    }
}

impl<D> From<Vec<Interval<D>>> for SortedDisjointIntervalList<D>
where
    D: Clone,
{
    fn from(intervals: Vec<Interval<D>>) -> Self {
        SortedDisjointIntervalList::new(intervals)
    }
}

impl<D> fmt::Display for SortedDisjointIntervalList<D>
where
    D: Clone + fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let range_strs = self.intervals.iter().map(|range| range.to_string());
        let str = range_strs.collect::<Vec<String>>().join(", ");
        write!(f, "[{}]", str)
    }
}

pub type BlankInterval = Interval<()>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval<D>
where
    D: Clone,
{
    range: ops::Range<i64>,
    data: Vec<D>,
}

impl<D> Interval<D>
where
    D: Clone,
{
    pub fn new(range: ops::Range<i64>) -> Self {
        Self {
            range,
            data: vec![],
        }
    }

    pub fn new_with_data(range: ops::Range<i64>, data: D) -> Self {
        Self {
            range,
            data: vec![data],
        }
    }

    fn intersect(&self, other: &Interval<D>) -> Option<Interval<D>> {
        let a = &self.range;
        let b = &other.range;

        let left = a.start.max(b.start);
        let right = a.end.min(b.end);

        if left < right {
            let mut combined_data = self.data.clone();
            combined_data.extend(other.data.clone());
            Some(Self {
                range: left..right,
                data: combined_data,
            })
        } else {
            None
        }
    }

    fn subtract(&self, other: &Interval<D>) -> Vec<Interval<D>> {
        match self.intersect(other) {
            // if there's some intersection, figure out which part to remove
            Some(intersection) => {
                let a = &self.range;
                let int = &intersection.range;

                if a.start == int.start && a.end == int.end {
                    // we are encompassed by the other range, so we are removed when subtracted
                    vec![]
                } else if a.start == int.start {
                    // the left side will be clipped off, since that's where the intersection aligns
                    vec![Self {
                        range: int.end..a.end,
                        data: self.data.clone(),
                    }]
                } else if a.end == int.end {
                    // the right side will be clipped off, since that's where the intersection aligns
                    vec![Self {
                        range: a.start..int.start,
                        data: self.data.clone(),
                    }]
                } else {
                    // we completely encompass the intersection, so we must segment ourself
                    vec![
                        Self {
                            range: a.start..int.start,
                            data: self.data.clone(),
                        },
                        Self {
                            range: int.end..a.end,
                            data: self.data.clone(),
                        },
                    ]
                }
            }
            // no intersection means nothing to subtract! :)
            None => vec![self.clone()],
        }
    }
}

// impl_op_ex!(&|a: &Interval<_>, b: &Interval| -> Option<Interval> { a.intersect(b) });

// impl_op_ex!(-|a: &Interval, b: &Interval| -> Option<Vec<Interval>> { a.subtract(b) });

impl<D> fmt::Display for Interval<D>
where
    D: Clone + fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?})", self.range)
    }
}
