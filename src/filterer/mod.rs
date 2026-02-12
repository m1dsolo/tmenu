pub mod contains_filterer;

pub use self::contains_filterer::ContainsFilterer;

pub trait Filterer<'a> {
    fn filter(&self, query: &str) -> Vec<&'a str>;
}
