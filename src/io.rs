use crate::{ exit, log };
use fuzzy_matcher::{ FuzzyMatcher, skim::SkimMatcherV2 };
use inquire::{ Text, type_aliases::Suggester, error::InquireError::* };
use itertools::Itertools;

pub fn prompt_user<T: AsRef<str>>(prompt: &str, suggestions: &[T]) -> String {
	let matcher = SkimMatcherV2::default();

	let suggester: Suggester = &|input| suggestions.into_iter()
		.map(|s| {
			let score = matcher.fuzzy_match(s.as_ref(), input)
				.unwrap_or_default(); // Score is 0 if there is no match
			(s, score)
		})
		.filter(|(_, score)| *score > 0)
		.sorted_by(|(_, a), (_, b)| b.cmp(a)) // Sort by score
		.map(|(s, _)| s.as_ref().to_string())
		.collect();

	let answer = Text::new(prompt)
		.with_suggester(suggester)
		.prompt();

	match answer {
		Ok(answer) => answer,
		Err(error) => match error {
			NotTTY | InvalidConfiguration(_) | IO(_) =>
				exit::with_error("Could not ask user for input"),
			_ =>
				exit::with_warn("Process stopped")
		}
	}
}

pub fn prompt_optional<T: AsRef<str>>(prompt: &str, suggestions: &[T]) -> String {
	prompt_user(&format!("(optional) {}", prompt), suggestions)
}

pub fn prompt_required<T: AsRef<str>>(prompt: &str, suggestions: &[T]) -> String {
	loop {
		let answer = prompt_user(&format!("(required) {}", prompt), suggestions);
		if !answer.is_empty() { return answer }
		log::info("This field cannot be empty");
	}
}

pub fn prompt_require_n<T: AsRef<str>>(
	prompt: &str,
	suggestions: &[T],
	n: usize
) -> Vec<String> {
	let mut answers = vec![];
	while answers.len() < n {
		answers.push(prompt_required(prompt, suggestions));
	}
	loop {
		let answer = prompt_optional(prompt, suggestions);
		if answer.is_empty() { break }
		answers.push(answer);
	}
	answers
}
