use super::Filterer;

pub struct ContainsFilterer<'a> {
    options: Vec<&'a str>,
}

impl<'a> ContainsFilterer<'a> {
    pub fn new(options: Vec<&'a str>) -> Self {
        Self { options }
    }
}

impl<'a> Filterer<'a> for ContainsFilterer<'a> {
    fn filter(&self, query: &str) -> Vec<&'a str> {
        self.options
            .iter()
            .filter(|option| option.contains(query))
            .cloned()
            .collect()
    }
}
