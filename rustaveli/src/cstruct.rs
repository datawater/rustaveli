use super::ctype::{CType, CTypeable};
use super::cvariable::CVariable;
use super::random_string::*;
use super::statics::*;

use rand::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CStruct {
    name: String,
    fields: Vec<(CType, String)>,
    number_of_fields: u8,
}

impl CTypeable for CStruct {
    fn to_string(&self) -> String {
        return self.name.clone();
    }

    fn default_value(&self) -> String {
        return "{".to_string()
            + &self
                .fields
                .clone()
                .into_iter()
                .map(|x| x.0.default_value())
                .collect::<Vec<_>>()
                .join(",")
            + "}";
    }

    fn random_value(&self) -> String {
        return "{".to_string()
            + &self
                .fields
                .clone()
                .into_iter()
                .map(|x| x.0.random_value())
                .collect::<Vec<_>>()
                .join(",")
            + "}";
    }

    fn size(&self) -> String {
        let cloned_fields = self.fields.clone();
        return "(".to_string()
            + &cloned_fields
                .into_iter()
                .filter(|x| PRIMITIVE_TYPES.contains(&x.0.value))
                .map(|x| x.0.size())
                .collect::<Vec<_>>()
                .join("+")
            + ")";
    }
}

impl CStruct {
    pub fn new(
        number_of_fields: u8,
        fields: Option<Vec<(CType, String)>>,
        name: Option<String>,
    ) -> Self {
        let mut rng = thread_rng();
        let name = name.unwrap_or(RandomString::new(Some(1), None).value);
        let fields = fields.unwrap_or(
            (0..number_of_fields)
                .map(|_| {
                    (
                        CType::new(
                            PRIMITIVE_TYPES.clone().into_iter().collect::<Vec<_>>()
                                [rng.gen_range(0..PRIMITIVE_TYPES.len()) as usize]
                                .clone(),
                        ),
                        RandomString::new(Some(0), Some(6)).value,
                    )
                })
                .collect(),
        );

        let mut r = Self::default();

        r.name = name;
        r.fields = fields;
        r.number_of_fields = number_of_fields;

        let default_value = r.default_value();
        let random: fn(Option<Box<dyn CTypeable>>) -> String = |x| x.unwrap().random_value();

        AVAILABLE_TYPES.insert(
            r.clone().name,
            TypeInformation {
                size: r.size(),
                default: default_value.clone(),
                random,
                compare: "CMP_STRUCT".to_string(),
                global_defenition: r.generate_defenition(),
                instructions: vec![],
                instruction_function: None,
                self_: Some(r.clone()),
            },
        );

        return r.clone();
    }

    pub fn generate_defenition(&self) -> String {
        return "typedef struct {\n    ".to_string()
            + &self
                .fields
                .clone()
                .into_iter()
                .map(|x| format!("{} {}", x.0.to_string(), x.1))
                .collect::<Vec<_>>()
                .join(";\n    ")
            + &format!(";\n}} {};\n", self.name);
    }

    pub fn apply_random_instruction_to_each_field(
        &self,
        rhs: &Self,
        self_name: &str,
        rhs_name: &str,
    ) -> String {
        let mut tr = format!("(({}) {{", self.name);

        for i in 0..self.fields.len() {
            let a = CVariable::new(
                Some(self.fields[i].0.to_string()),
                Some(&(self_name.to_string() + "." + &self.fields[i].1)),
                None,
            );

            let b = CVariable::new(
                Some(rhs.fields[i].0.to_string()),
                Some(&(rhs_name.to_string() + "." + &rhs.fields[i].1)),
                None,
            );

            tr += &(a.apply_operation(&b, None) + ", ");
        }

        tr.pop();
        tr.pop();
        tr += "})";

        return tr;
    }
}
