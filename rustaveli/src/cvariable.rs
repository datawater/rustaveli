use crate::ctype::*;
use crate::random_string::*;
use crate::statics::*;
use rand::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CVariable {
    pub(crate) type_: CType,
    pub(crate) name: String,
    pub(crate) value: String,
}

impl CVariable {
    pub fn new(type_: Option<String>, name: Option<&str>, value: Option<&str>) -> Self {
        let mut rt = Self::default();

        let mut rng = thread_rng();

        rt.type_ = CType::new(
            type_.unwrap_or(
                AVAILABLE_TYPES
                    .iter()
                    .map(|x| x.key().clone())
                    .collect::<Vec<_>>()[rng.gen_range(0..AVAILABLE_TYPES.len()) as usize]
                    .clone(),
            ),
        );

        rt.name = name
            .unwrap_or(&RandomString::new(None, None).value)
            .to_string();
        rt.value = value.unwrap_or(&rt.type_.random_value()).to_string();

        return rt;
    }

    pub fn generate_defenition(&self) -> String {
        return format!(
            "volatile {} {} = {};",
            self.type_.to_string(),
            self.name,
            self.value
        );
    }

    pub fn apply_operation(&self, rhs: &Self, op: Option<String>) -> String {
        if !PRIMITIVE_TYPES.contains(&self.type_.to_string()) && op.is_some() {
            eprintln!(
                "[WARN] Structs can only have random operators applied. Returning an empty string"
            );
            return String::new();
        }

        if !PRIMITIVE_TYPES.contains(&self.type_.to_string()) {
            if self.type_.to_string() != rhs.type_.to_string() {
                eprintln!("[WARN] Struct types don't match. Returning an empty string");
                return String::new();
            }

            return AVAILABLE_TYPES
                .get(&self.type_.to_string())
                .unwrap()
                .self_
                .clone()
                .unwrap()
                .apply_random_instruction_to_each_field(
                    &AVAILABLE_TYPES
                        .get(&rhs.type_.to_string())
                        .unwrap()
                        .self_
                        .clone()
                        .unwrap(),
                    &self.name,
                    &rhs.name,
                );
        }

        let mut op = op.unwrap_or_default();

        if op.is_empty() {
            let mut n = 0;

            let self_instructions = &AVAILABLE_TYPES
                .get(&self.type_.to_string())
                .unwrap()
                .instructions;
            let rhs_instructions = &AVAILABLE_TYPES
                .get(&rhs.type_.to_string())
                .unwrap()
                .instructions;

            let mut rng = thread_rng();

            while !self_instructions.contains(&op) || !rhs_instructions.contains(&op) {
                op = self_instructions[rng.gen_range(0..self_instructions.len()) as usize].clone();

                n += 1;

                if n > 10 {
                    eprintln!("[WARN] Operation finding iteration maximum reached. Returning an empty string");
                    return String::new();
                }
            }
        }

        match op.as_str() {
            "append" | "dup" => {
                return AVAILABLE_TYPES
                    .get("char*")
                    .unwrap()
                    .instruction_function
                    .unwrap()(self.name.clone(), Some(rhs.name.clone()), op);
            }

            _ => {
                return format!(
                    "({}) ({} {} {})",
                    self.type_.to_string(),
                    self.name,
                    op,
                    rhs.name
                )
            }
        }
    }
}
