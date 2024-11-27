use crate::ctype::*;
use crate::cvariable::*;
use crate::random_string::*;
use crate::statics::*;

use rand::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CUtilityFunction {
    pub return_type: String,
    pub accept_n: u8,
    pub accept: Vec<CVariable>,
    pub name: String,
    code: String,
}

impl CUtilityFunction {
    pub fn new(
        return_type: Option<String>,
        accept_n: Option<u8>,
        accept: Option<Vec<CVariable>>,
        name: Option<String>,
    ) -> Self {
        let mut rt = Self::default();

        let keys = AVAILABLE_TYPES
            .iter()
            .map(|x| x.key().clone())
            .collect::<Vec<_>>();

        let mut rng = thread_rng();

        rt.return_type = return_type.unwrap_or(keys[rng.gen_range(0..keys.len())].clone());

        rt.accept_n = accept_n.unwrap_or(rng.gen_range(0..5) as u8);
        rt.accept = accept.unwrap_or(
            (0..rt.accept_n)
                .map(|_| CVariable::new(None, None, None))
                .collect(),
        );
        rt.name = name.unwrap_or(RandomString::new(None, None).value);

        rt.code += &rt.generate_defenition();
        rt.code += &rt.generate_body();

        return rt;
    }

    fn generate_defenition(&self) -> String {
        let args = self
            .accept
            .iter()
            .map(|x| format!("{} {}", x.type_.to_string(), x.name.clone()))
            .collect::<Vec<_>>()
            .join(", ");

        return format!("{} {}(", self.return_type, self.name) + &args + ")";
    }

    fn generate_body(&self) -> String {
        let mut tr = "{\n    ".to_string();

        let tr_variable = CVariable::new(Some(self.return_type.clone()), Some("tr"), None);
        tr += &(tr_variable.generate_defenition() + "\n    ");

        let mut rng = thread_rng();

        for _ in 0..rng.gen_range(1..8) {
            let apply_tr = CVariable::new(Some(self.return_type.clone()), None, None);
            tr += &(apply_tr.generate_defenition() + "\n");
            tr += &("    tr = ".to_string()
                + &tr_variable.apply_operation(&apply_tr, None)
                + ";\n    ");
        }

        tr += "\n    ";

        for _ in 0..rng.gen_range(0..16) {
            let op_variable = CVariable::new(None, None, None);

            tr += &(op_variable.generate_defenition() + "\n    ");

            for _ in 1..4 {
                let apply_tr = CVariable::new(Some(op_variable.type_.to_string()), None, None);
                tr += &(apply_tr.generate_defenition() + "\n    ");
                tr += &format!(
                    "{} = {};\n    ",
                    op_variable.name,
                    op_variable.apply_operation(&apply_tr, None)
                );
            }

            tr += "\n    ";
        }

        tr += "return tr;\n";
        tr += "}";

        return tr;
    }

    pub fn finish(&self) -> &str {
        return &self.code;
    }
}
