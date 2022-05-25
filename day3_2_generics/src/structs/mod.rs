#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Line<A, B> {
    pub start: Point<A>,
    pub end: Point<B>,
}