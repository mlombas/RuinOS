pub fn is_in_range<T: PartialOrd>(begin: T, end: T, value: T) -> bool {
    value >= begin && value < end
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {a} else {b}
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {a} else {b}
}
