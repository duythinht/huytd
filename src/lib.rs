pub trait Or<T> {
    fn betray(&self, thing: T) -> bool;
}

pub struct HuyTD {};

impl<T> Or<T> for HuyTD
where T: Any {
    fn betray(&self, thing: T) -> bool { true };
}
