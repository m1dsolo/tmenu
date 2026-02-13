pub mod contains_filterer;
pub mod fuzzy_filterer;

pub use self::contains_filterer::ContainsFilterer;
pub use self::fuzzy_filterer::FuzzyFilterer;

pub trait Filterer<'a> {
    fn filter(&self, query: &str) -> Vec<&'a str>;
}
