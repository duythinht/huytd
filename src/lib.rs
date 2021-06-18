pub trait Or<T> {
    fn betray(&self, thing: T) -> bool;
}