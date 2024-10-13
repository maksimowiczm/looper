use crate::algorithm::mutator::best_mutator::BestMutator;
use crate::algorithm::mutator::current_mutator::CurrentMutator;
use crate::algorithm::mutator::random_mutator::RandomMutator;
use crate::algorithm::mutator::Mutate;
use once_cell::sync::Lazy;
use regex::Regex;

pub struct UnknownMutator;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(DE/)?(?<x>[a-zA-Z]+)/(?<how_many>[1-9]\d?)/bin").expect("This is a valid regex")
});

pub fn parse_mutator(mutator: &str) -> Result<Box<dyn Mutate>, UnknownMutator> {
    let captures = REGEX.captures(mutator).ok_or(UnknownMutator)?;

    let x = captures.name("x").ok_or(UnknownMutator)?.as_str(); // special thanks to our professor for naming this variable (it a satire please don't fail us 😔)
    let how_many = captures
        .name("how_many")
        .and_then(|m| m.as_str().parse::<usize>().ok())
        .ok_or(UnknownMutator)?;

    let mutator: Box<dyn Mutate> = match x {
        "current" => Box::new(CurrentMutator { size: how_many }),
        "best" => Box::new(BestMutator { size: how_many }),
        "rand" => Box::new(RandomMutator { size: how_many }),
        _ => return Err(UnknownMutator),
    };

    Ok(mutator)
}
