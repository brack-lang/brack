use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    TInline,
    TOption(Box<Type>),
    TBlock,
    TArray(Box<Type>),
    TInlineCmd(String),
    TBlockCmd(String),
    TAST,
}

pub fn arg_counter(arg_types: &Vec<Type>) -> (usize, usize) {
    let mut min = 0;
    let mut max = 0;
    for arg_type in arg_types {
        match arg_type {
            Type::TOption(_) => {
                min += 0;
                max += 1;
            }
            Type::TArray(_) => {
                min += 0;
                max = usize::MAX;
            }
            _ => {
                min += 1;
                max += 1;
            }
        }
    }
    (min, max)
}
