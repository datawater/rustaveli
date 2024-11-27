use std::collections::HashSet;
use std::sync::Arc;

use dashmap::DashMap;
use lazy_static::lazy_static;
use rand::prelude::*;

use super::{cstruct::CStruct, ctype::CTypeable, random_string::RandomString};

lazy_static! {
    pub(crate) static ref PRIMITIVE_TYPES: HashSet<String> = HashSet::from([
        "bool".to_string(),
        "int".to_string(),
        "char*".to_string(),
        "float".to_string(),
        "double".to_string(),
        "size_t".to_string(),
    ]);
}

type InstructionCompletionFunction = fn(String, Option<String>, String) -> String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TypeInformation {
    pub(crate) size: String,
    pub(crate) default: String,
    pub(crate) random: fn(Option<Box<dyn CTypeable>>) -> String,
    pub(crate) compare: String,
    pub(crate) global_defenition: String,
    pub(crate) instructions: Vec<String>,
    pub(crate) instruction_function: Option<InstructionCompletionFunction>,
    pub(crate) self_: Option<CStruct>,
}

lazy_static! {
    pub(crate) static ref AVAILABLE_TYPES: Arc<DashMap<String, TypeInformation>> =
        Arc::new(DashMap::from_iter([
            (
                "bool".to_string(),
                TypeInformation {
                    size: "sizeof(bool)".into(),
                    default: "false".into(),
                    random: |_s| return random::<bool>().to_string(),
                    compare: "CMP_INT".into(),
                    global_defenition: "".into(),
                    instructions: vec!["&".into(), "|".into(), "^".into()],
                    instruction_function: None,
                    self_: None,
                },
            ),
            (
                "int".to_string(),
                TypeInformation {
                    size: "sizeof(int)".into(),
                    default: "0".into(),
                    random: |_s| {
                        return random::<i32>().to_string();
                    },
                    compare: "CMP_INT".into(),
                    global_defenition: "".into(),
                    instructions: vec![
                        "+".into(),
                        "-".into(),
                        "*".into(),
                        "/".into(),
                        "%".into(),
                        "&".into(),
                        "|".into(),
                        "^".into(),
                    ],
                    instruction_function: None,
                    self_: None,
                },
            ),
            (
                "size_t".to_string(),
                TypeInformation {
                    size: "sizeof(size_t)".into(),
                    default: "0".into(),
                    random: |_s| return random::<usize>().to_string(),
                    compare: "CMP_SIZE_T".into(),
                    global_defenition: "".into(),
                    instructions: vec![
                        "+".into(),
                        "-".into(),
                        "*".into(),
                        "/".into(),
                        "%".into(),
                        "&".into(),
                        "|".into(),
                        "^".into(),
                    ],
                    instruction_function: None,
                    self_: None,
                },
            ),
            (
                "float".to_string(),
                TypeInformation {
                    size: "sizeof(float)".into(),
                    default: "0.0".into(),
                    random: |_s| return random::<f32>().to_string(),
                    compare: "CMP_FLOAT".into(),
                    global_defenition: "".into(),
                    instructions: vec!["+".into(), "-".into(), "*".into(), "/".into()],
                    instruction_function: None,
                    self_: None,
                },
            ),
            (
                "double".to_string(),
                TypeInformation {
                    size: "sizeof(double)".into(),
                    default: "0.0".into(),
                    random: |_s| return random::<f64>().to_string(),
                    compare: "CMP_DOUBLE".into(),
                    global_defenition: "".into(),
                    instructions: vec!["+".into(), "-".into(), "*".into(), "/".into()],
                    instruction_function: None,
                    self_: None,
                },
            ),
            (
                "char*".to_string(),
                TypeInformation {
                    size: "sizeof(char*)".into(),
                    default: "0.0".into(),
                    random: |_s| return format!("\"{}\"", RandomString::new(None, None).value),
                    compare: "CMP_STRING".into(),
                    global_defenition: "".into(),
                    instructions: vec!["append".into(), "dup".into()],
                    instruction_function: Some(|a, b, i| {
                        if i == "append" {
                            format!("__strcat({a}, {})", b.unwrap())
                        } else {
                            format!("__strdup({a})")
                        }
                    }),
                    self_: None,
                },
            ),
        ]));
}
