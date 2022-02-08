use fuzzy_matcher::{ FuzzyMatcher, skim::SkimMatcherV2 };
use inquire::{ Text, type_aliases::Suggester };

pub fn prompt(prompt: &str, suggestions: &[&str]) -> String {
	let matcher = SkimMatcherV2::default();

    let suggester: Suggester = &|input| {
		let mut suggestions: Vec<(&&str, i64)> = suggestions.iter()
			// Fuzzy match with user input
			.map(|i| (i, matcher.fuzzy_match(i, input).unwrap_or_default()))
			// Remove unmatching suggestions
			.filter(|(_, score)| *score > 0)
			.collect();
		// Sort by score
		suggestions.sort_by(|(_, a), (_, b)| b.cmp(a));
		suggestions.iter().map(|(i, _)| i.to_string()).collect()
	};

	let answer = Text::new(prompt)
		.with_suggester(suggester)
		.prompt();

	match answer {
		Ok(answer) => answer,
		Err(_) => std::process::exit(1)
	}
}
