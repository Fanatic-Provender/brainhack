#[derive(Parser)]
#[grammar = "bin/assembler/hack.pest"]
pub struct HackParser;

pub type HackPair<'i> = pest::iterators::Pair<'i, Rule>;
