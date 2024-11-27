use crate::ctype::*;
use crate::cutilityfunction::*;
use crate::cvariable::*;
use crate::random_string::*;

use rand::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CRunningFunction {
    pub name: String,
    code: String,
    op_type: CVariable,
    util_functions_n: u8,
    util_functions: Vec<CUtilityFunction>,
}

impl CRunningFunction {
    pub fn new(name: Option<String>) -> Self {
        let mut rt = Self::default();
        let mut rng = thread_rng();

        rt.name = name.unwrap_or(RandomString::new(None, None).value);
        rt.op_type = CVariable::new(None, None, None);
        rt.util_functions_n = rng.gen_range(1..=4);
        rt.util_functions = (0..rt.util_functions_n)
            .map(|_| CUtilityFunction::new(Some(rt.op_type.type_.to_string()), None, None, None))
            .collect();

        rt.code += &rt
            .util_functions
            .iter()
            .map(|x| x.finish().to_string() + "\n")
            .collect::<Vec<_>>()
            .join("\n");

        rt.code += &rt.generate_defenition();
        rt.generate_body();

        return rt;
    }

    fn generate_defenition(&self) -> String {
        return format!("__attribute__((constructor)) void {}()", self.name);
    }

    fn generate_body(&mut self) {
        self.code += " {\n    ";
        self.code += &(self.op_type.generate_defenition() + "\n\n    ");

        for util_function in &self.util_functions {
            for accept in &util_function.accept {
                self.code += &(accept.generate_defenition() + "\n    ");
            }

            let accepts = util_function
                .accept
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            let result = CVariable::new(
                Some(self.op_type.type_.to_string()),
                Some(&(util_function.name.clone() + "_v")),
                Some(&format!("{}({accepts})", util_function.name)),
            );

            self.code += &(result.generate_defenition() + "\n    ");
            self.code += &format!(
                "{} = {};",
                self.op_type.name,
                self.op_type.apply_operation(&result, None)
            );
            self.code += "\n\n    ";
        }

        self.code = self.code[0..self.code.len() - 5].to_string();
        self.code += "}\n\n";
    }

    pub(crate) fn finish(&self) -> &str {
        return &self.code;
    }
}
