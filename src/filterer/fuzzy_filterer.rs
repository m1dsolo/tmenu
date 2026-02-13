use super::Filterer;

pub struct FuzzyFilterer<'a> {
    options: Vec<&'a str>,
}

#[derive(Clone)]
pub struct MatchResult<'a> {
    pub text: &'a str,
    pub matched_indices: Vec<usize>,
}

impl<'a> FuzzyFilterer<'a> {
    pub fn new(options: Vec<&'a str>) -> Self {
        Self { options }
    }

    fn fuzzy_match(&self, text: &str, pattern: &str) -> Option<(usize, Vec<usize>)> {
        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        if pattern_chars.is_empty() {
            return Some((0, vec![]));
        }

        let mut score = 0;
        let mut prev_idx = None;
        let mut matched_indices = Vec::new();
        let mut used_indices = vec![false; text_chars.len()];

        for (i, pc) in pattern_chars.iter().enumerate() {
            let mut found = false;

            for (j, tc) in text_chars.iter().enumerate() {
                if !used_indices[j] && pc.eq_ignore_ascii_case(tc) {
                    if i == 0 {
                        score += 10;
                    } else if let Some(prev) = prev_idx {
                        if j == prev + 1 {
                            score += 5;
                        } else if j > prev {
                            score += 1;
                        }
                    }

                    if j == 0 {
                        score += 2;
                    }

                    used_indices[j] = true;
                    prev_idx = Some(j);
                    matched_indices.push(j);
                    found = true;
                    break;
                }
            }

            if !found {
                return None;
            }
        }

        Some((score, matched_indices))
    }

    pub fn filter_with_matches(&self, query: &str) -> Vec<MatchResult<'a>> {
        if query.is_empty() {
            return self
                .options
                .iter()
                .map(|&text| MatchResult {
                    text,
                    matched_indices: vec![],
                })
                .collect();
        }

        let mut results: Vec<(&str, usize, Vec<usize>)> = self
            .options
            .iter()
            .filter_map(|option| {
                self.fuzzy_match(option, query)
                    .map(|(score, indices)| (*option, score, indices))
            })
            .collect();

        results.sort_by(|a, b| b.1.cmp(&a.1));

        results
            .into_iter()
            .map(|(text, _, indices)| MatchResult {
                text,
                matched_indices: indices,
            })
            .collect()
    }
}

impl<'a> Filterer<'a> for FuzzyFilterer<'a> {
    fn filter(&self, query: &str) -> Vec<&'a str> {
        self.filter_with_matches(query)
            .into_iter()
            .map(|r| r.text)
            .collect()
    }
}
