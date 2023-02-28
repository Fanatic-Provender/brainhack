use {
    crate::parser::{HackPair, Rule},
    anyhow::{anyhow, bail},
    brainhack::prelude::*,
    itertools::{chain, Itertools},
    std::{
        collections::{hash_map::Entry, HashMap},
        io::Write,
    },
};

#[derive(Clone, Debug)]
pub struct SymbolData {
    pub value: usize,
    pub is_predefined: bool,
}

type SymbolTable = HashMap<String, SymbolData>;

const RESERVED_REGISTERS: usize = 16;
const ADDRESS_SPACE_SIZE: usize = 32768;
static KEYWORDS: &[(&str, usize)] = &[
    ("SP", 0),
    ("LCL", 1),
    ("ARG", 2),
    ("THIS", 3),
    ("THAT", 4),
    ("SCREEN", 16384),
    ("KBD", 24576),
];

pub fn assemble<W: Write>(file: HackPair, out: W) -> anyhow::Result<W> {
    let mut coder = Coder::new(out);
    let symbol_table = scan_symbols(file.clone())?;

    coder.while_cond(
        pos::FU,
        |c| {
            c.copy_word(word::P, &[word::Q], pos::VU)?
                .inc_word(word::Q, [pos::VU, pos::VL])?
                .is_nonzero_move(word::Q, pos::FU)
        },
        |c| {
            c.clear_cell(&[pos::FU])?
                .copy_word(word::P, &[word::Q], pos::VU)?;

            for line in file.into_inner() {
                match line.as_rule() {
                    Rule::a_instruction => {
                        let spec = line.into_inner().exactly_one().unwrap();

                        let value = match spec.as_rule() {
                            Rule::constant => {
                                let spec = spec.as_str();
                                let value = spec
                                    .parse()
                                    .map_err(|_| anyhow!("invalid constant '{}'", spec))?;
                                if (0..ADDRESS_SPACE_SIZE).contains(&value) {
                                    value
                                } else {
                                    bail!("invalid constant '{}'", spec)
                                }
                            }
                            Rule::symbol => {
                                symbol_table
                                    .get(spec.as_str())
                                    .expect("incomplete symbol table")
                                    .value
                            }
                            _ => unreachable!(),
                        };
                        let value = u16::try_from(value)
                            .map_err(|_| anyhow!("invalid constant '{}'", spec))?;

                        c.inc_word(word::Q, [pos::VU, pos::VL])?
                            .is_nonzero(word::Q, pos::FU, [pos::VU, pos::VL])?
                            .if_else_move(
                                pos::FU,
                                pos::FL,
                                |c| {
                                    c.dec_word(word::Q, [pos::VU, pos::VL])?
                                        .is_zero(word::Q, pos::FU, [pos::VU, pos::VL])?
                                        .if_move(pos::FU, |c| {
                                            c.set_word(word::A, value)?.seek(5)?.write("#")
                                        })
                                },
                                |c| c.dec_word(word::Q, [pos::VU, pos::VL]),
                            )?;
                    }
                    Rule::c_instruction => {
                        let mut dest = "";
                        let mut comp = "";
                        let mut jump = "";

                        for spec in line.into_inner() {
                            match spec.as_rule() {
                                Rule::dest => dest = spec.as_str(),
                                Rule::comp => comp = spec.as_str(),
                                Rule::jump => jump = spec.as_str(),
                                _ => unreachable!(),
                            }
                        }

                        c.inc_word(word::Q, [pos::VU, pos::VL])?
                            .is_nonzero(word::Q, pos::FU, [pos::VU, pos::VL])?
                            .if_else_move(
                                pos::FU,
                                pos::FL,
                                |c| {
                                    c.dec_word(word::Q, [pos::VU, pos::VL])?
                                        .is_zero(word::Q, pos::FU, [pos::VU, pos::VL])?
                                        .if_move(pos::FU, |c| {
                                            if comp.contains('M') {
                                                c.clear_cell(&[pos::MU, pos::ML])?.read_memory()?;
                                            }

                                            match comp {
                                                "0" => {
                                                    c.set_word(word::R, 0)?;
                                                }
                                                "1" => {
                                                    c.set_word(word::R, 1)?;
                                                }
                                                "-1" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .seek(pos::RU)?
                                                        .dec_val()?
                                                        .seek(pos::RL)?
                                                        .dec_val()?;
                                                }
                                                "D" => {
                                                    c.copy_word(word::D, &[word::R], pos::VU)?;
                                                }
                                                "A" => {
                                                    c.copy_word(word::A, &[word::R], pos::VU)?;
                                                }
                                                "M" => {
                                                    c.copy_word(word::M, &[word::R], pos::VU)?;
                                                }
                                                "-D" => {
                                                    c.sub_word(
                                                        word::D,
                                                        word::R,
                                                        [
                                                            pos::VU,
                                                            pos::VL,
                                                            pos::T7,
                                                            pos::WU,
                                                            pos::WL,
                                                        ],
                                                    )?;
                                                }
                                                "-A" => {
                                                    c.sub_word(
                                                        word::A,
                                                        word::R,
                                                        [
                                                            pos::VU,
                                                            pos::VL,
                                                            pos::T7,
                                                            pos::WU,
                                                            pos::WL,
                                                        ],
                                                    )?;
                                                }
                                                "-M" => {
                                                    c.sub_word(
                                                        word::M,
                                                        word::R,
                                                        [
                                                            pos::VU,
                                                            pos::VL,
                                                            pos::T7,
                                                            pos::WU,
                                                            pos::WL,
                                                        ],
                                                    )?;
                                                }
                                                "D+1" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::D, &[word::R], pos::VU)?
                                                        .inc_word(word::R, [pos::VU, pos::VL])?;
                                                }
                                                "A+1" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::A, &[word::R], pos::VU)?
                                                        .inc_word(word::R, [pos::VU, pos::VL])?;
                                                }
                                                "M+1" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::M, &[word::R], pos::VU)?
                                                        .inc_word(word::R, [pos::VU, pos::VL])?;
                                                }
                                                "D-1" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::D, &[word::R], pos::VU)?
                                                        .dec_word(word::R, [pos::VU, pos::VL])?;
                                                }
                                                "A-1" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::A, &[word::R], pos::VU)?
                                                        .dec_word(word::R, [pos::VU, pos::VL])?;
                                                }
                                                "M-1" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::M, &[word::R], pos::VU)?
                                                        .dec_word(word::R, [pos::VU, pos::VL])?;
                                                }
                                                "D+A" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::D, &[word::R], pos::VU)?
                                                        .add_word(
                                                            word::A,
                                                            word::R,
                                                            [
                                                                pos::VU,
                                                                pos::VL,
                                                                pos::T7,
                                                                pos::WU,
                                                                pos::WL,
                                                            ],
                                                        )?;
                                                }
                                                "D+M" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::D, &[word::R], pos::VU)?
                                                        .add_word(
                                                            word::M,
                                                            word::R,
                                                            [
                                                                pos::VU,
                                                                pos::VL,
                                                                pos::T7,
                                                                pos::WU,
                                                                pos::WL,
                                                            ],
                                                        )?;
                                                }
                                                "D-A" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::D, &[word::R], pos::VU)?
                                                        .sub_word(
                                                            word::A,
                                                            word::R,
                                                            [
                                                                pos::VU,
                                                                pos::VL,
                                                                pos::T7,
                                                                pos::WU,
                                                                pos::WL,
                                                            ],
                                                        )?;
                                                }
                                                "D-M" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::D, &[word::R], pos::VU)?
                                                        .sub_word(
                                                            word::M,
                                                            word::R,
                                                            [
                                                                pos::VU,
                                                                pos::VL,
                                                                pos::T7,
                                                                pos::WU,
                                                                pos::WL,
                                                            ],
                                                        )?;
                                                }
                                                "A-D" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::A, &[word::R], pos::VU)?
                                                        .sub_word(
                                                            word::D,
                                                            word::R,
                                                            [
                                                                pos::VU,
                                                                pos::VL,
                                                                pos::T7,
                                                                pos::WU,
                                                                pos::WL,
                                                            ],
                                                        )?;
                                                }
                                                "M-D" => {
                                                    c.clear_cell(&[pos::RU, pos::RL])?
                                                        .copy_word(word::M, &[word::R], pos::VU)?
                                                        .sub_word(
                                                            word::D,
                                                            word::R,
                                                            [
                                                                pos::VU,
                                                                pos::VL,
                                                                pos::T7,
                                                                pos::WU,
                                                                pos::WL,
                                                            ],
                                                        )?;
                                                }
                                                _ => todo!(
                                                    "unsupported comp specification '{}'",
                                                    comp
                                                ),
                                            }

                                            let dest_words: Vec<_> = chain!(
                                                dest.contains('A').then_some(word::A),
                                                dest.contains('D').then_some(word::D),
                                                dest.contains('M').then_some(word::M),
                                            )
                                            .collect();
                                            c.clear_cell(
                                                &dest_words
                                                    .iter()
                                                    .flat_map(|&(u, l)| [u, l])
                                                    .collect_vec(),
                                            )?
                                            .copy_word(word::R, &dest_words, pos::VU)?;

                                            if dest.contains('M') {
                                                c.write_memory()?;
                                            }

                                            match jump {
                                                "" => {}
                                                "JMP" => {
                                                    c.clear_cell(&[pos::PU, pos::PL])?
                                                        .copy_word(word::A, &[word::P], pos::VU)?
                                                        .dec_word(word::P, [pos::VU, pos::VL])?;
                                                }
                                                "JEQ" => {
                                                    c.is_zero(
                                                        word::R,
                                                        pos::FU,
                                                        [pos::VU, pos::VL],
                                                    )?
                                                    .if_move(pos::FU, |c| {
                                                        c.clear_cell(&[pos::PU, pos::PL])?
                                                            .copy_word(
                                                                word::A,
                                                                &[word::P],
                                                                pos::VU,
                                                            )?
                                                            .dec_word(word::P, [pos::VU, pos::VL])
                                                    })?;
                                                }
                                                "JNE" => {
                                                    c.is_nonzero(
                                                        word::R,
                                                        pos::FU,
                                                        [pos::VU, pos::VL],
                                                    )?
                                                    .if_move(pos::FU, |c| {
                                                        c.clear_cell(&[pos::PU, pos::PL])?
                                                            .copy_word(
                                                                word::A,
                                                                &[word::P],
                                                                pos::VU,
                                                            )?
                                                            .dec_word(word::P, [pos::VU, pos::VL])
                                                    })?;
                                                }
                                                "JLT" => {
                                                    c.is_lt_zero(
                                                        word::R,
                                                        pos::FU,
                                                        [pos::VU, pos::VL, pos::WU, pos::WL],
                                                    )?
                                                    .if_move(pos::FU, |c| {
                                                        c.clear_cell(&[pos::PU, pos::PL])?
                                                            .copy_word(
                                                                word::A,
                                                                &[word::P],
                                                                pos::VU,
                                                            )?
                                                            .dec_word(word::P, [pos::VU, pos::VL])
                                                    })?;
                                                }
                                                "JGT" => {
                                                    c.is_gt_zero(
                                                        word::R,
                                                        pos::FU,
                                                        [
                                                            pos::VU,
                                                            pos::VL,
                                                            pos::T7,
                                                            pos::WU,
                                                            pos::WL,
                                                            pos::T8,
                                                        ],
                                                    )?
                                                    .if_move(pos::FU, |c| {
                                                        c.clear_cell(&[pos::PU, pos::PL])?
                                                            .copy_word(
                                                                word::A,
                                                                &[word::P],
                                                                pos::VU,
                                                            )?
                                                            .dec_word(word::P, [pos::VU, pos::VL])
                                                    })?;
                                                }
                                                "JLE" => {
                                                    c.is_le_zero(
                                                        word::R,
                                                        pos::FU,
                                                        [
                                                            pos::VU,
                                                            pos::VL,
                                                            pos::T7,
                                                            pos::WU,
                                                            pos::WL,
                                                            pos::T8,
                                                        ],
                                                    )?
                                                    .if_move(pos::FU, |c| {
                                                        c.clear_cell(&[pos::PU, pos::PL])?
                                                            .copy_word(
                                                                word::A,
                                                                &[word::P],
                                                                pos::VU,
                                                            )?
                                                            .dec_word(word::P, [pos::VU, pos::VL])
                                                    })?;
                                                }
                                                "JGE" => {
                                                    c.is_ge_zero(
                                                        word::R,
                                                        pos::FU,
                                                        [pos::VU, pos::VL, pos::WU, pos::WL],
                                                    )?
                                                    .if_move(pos::FU, |c| {
                                                        c.clear_cell(&[pos::PU, pos::PL])?
                                                            .copy_word(
                                                                word::A,
                                                                &[word::P],
                                                                pos::VU,
                                                            )?
                                                            .dec_word(word::P, [pos::VU, pos::VL])
                                                    })?;
                                                }
                                                _ => unreachable!(),
                                            }

                                            c.clear_cell(&[pos::RU, pos::RL])?.seek(5)?.write("!")
                                        })
                                },
                                |c| c.dec_word(word::Q, [pos::VU, pos::VL]),
                            )?;
                    }
                    Rule::label_definition => {}
                    Rule::EOI => {
                        return c.is_zero_move(word::Q, pos::FU, pos::VU)?.if_else_move(
                            pos::FU,
                            pos::FL,
                            |c| {
                                c.clear_cell(&[pos::PU, pos::PL])?
                                    .seek(pos::PU)?
                                    .dec_val()?
                                    .seek(pos::PL)?
                                    .dec_val()
                            },
                            |c| c.inc_word(word::P, [pos::VU, pos::VL]),
                        )
                    }
                    _ => unreachable!(),
                }

                c.dec_word(word::Q, [pos::VU, pos::VL])?;
            }

            unreachable!()
        },
    )?;

    Ok(coder.into_writer())
}

pub fn scan_symbols(file: HackPair) -> anyhow::Result<SymbolTable> {
    let mut symbol_table: SymbolTable = (0..RESERVED_REGISTERS)
        .map(|i| {
            (
                format!("R{i}"),
                SymbolData {
                    value: i,
                    is_predefined: true,
                },
            )
        })
        .chain(KEYWORDS.iter().map(|&(s, v)| {
            (
                s.to_owned(),
                SymbolData {
                    value: v,
                    is_predefined: true,
                },
            )
        }))
        .collect();

    // scan labels
    let mut line_number = 0;

    for line in file.clone().into_inner() {
        match line.as_rule() {
            Rule::a_instruction | Rule::c_instruction => {
                if line_number >= ADDRESS_SPACE_SIZE {
                    bail!("too many lines");
                }
                line_number += 1;
            }
            Rule::label_definition => {
                let symbol = line.into_inner().exactly_one().unwrap().as_str();
                match symbol_table.entry(symbol.to_owned()) {
                    Entry::Occupied(entry) => {
                        if entry.get().is_predefined {
                            bail!("symbol '{symbol}' is predefined");
                        } else {
                            bail!("symbol '{symbol}' is already defined");
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(SymbolData {
                            value: line_number,
                            is_predefined: false,
                        });
                    }
                }
            }
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    // scan variables
    let mut stack_top = RESERVED_REGISTERS;

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::a_instruction => {
                let inner = line.into_inner().exactly_one().unwrap();

                match inner.as_rule() {
                    Rule::constant => {}
                    Rule::symbol => {
                        let symbol = inner.as_str();
                        if symbol_table.contains_key(symbol) {
                            continue;
                        }
                        if stack_top >= ADDRESS_SPACE_SIZE {
                            bail!("too many variables");
                        }

                        symbol_table.insert(
                            symbol.to_owned(),
                            SymbolData {
                                value: stack_top,
                                is_predefined: false,
                            },
                        );
                        stack_top += 1;
                    }
                    _ => unreachable!(),
                }
            }
            Rule::c_instruction | Rule::label_definition => {}
            Rule::EOI => return Ok(symbol_table),
            _ => unreachable!(),
        }
    }

    unreachable!()
}
