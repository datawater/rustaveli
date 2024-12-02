use crate::crunningfunction::*;
use crate::cstruct::*;
use crate::statics::*;

use rand::prelude::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RandomCFile {
    number_of_functions: u8,
    generated_functions: Vec<String>,
    code: String,
}

static STARTING_C_CODE: &str = "// #include <math.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

// #define FLT_EPSILON 0.0001
// #define MAX(a, b) a > b ? (a) : (b)
// #define CMP(a, b) ((a) == (b))
// #define CMP_INT(a, b) CMP(a, b)
// #define CMP_FLOAT(a, b) (fabs(CMP(a, b)) <= FPS_EPSILON * max(fabs((a)), fabs((b))))
// #define CMP_STR(a, b) (strcmp((a), (b)))
// #define CMP_BOOL(a, b) CMP(a, b)
// #define CMP_SIZE_T(a, b) CMP(a, b)
// #define CMP_STRUCT(a, b) (memcmp((a), (b), sizeof(a)))

__attribute__((noreturn)) volatile void __assert(bool x) {
    if (!x)
        *NULL;
}

volatile char* __strdup(char* a) {
    size_t l = strlen(a);
    char* r = (char*) malloc(sizeof(char) * l);
    __assert(r != NULL);
    memcpy(r, a, sizeof(char) * l);
    return r;
}

volatile char* __strcat(char* a, char* b) {
    size_t al = strlen(a);
    size_t bl = strlen(b);
    char* r = (char*) malloc(sizeof(char) * (al + bl));
    __assert(r != NULL);
    memcpy(r, a, sizeof(char) * al);
    memcpy(r + (sizeof(char) * al), b, sizeof(char) * bl);
    return r;
}

";

impl RandomCFile {
    pub fn new(number_of_functions: u8, number_of_structs_to_generate: u8) -> Self {
        let mut rt = Self::default();
        rt.number_of_functions = number_of_functions;
        rt.code = STARTING_C_CODE.to_string();

        let mut rng = thread_rng();

        for _ in 0..number_of_structs_to_generate {
            let a = CStruct::new(rng.gen_range(1..12) as u8, None, None);

            rt.code += &(a.generate_defenition() + "\n");
        }

        AVAILABLE_TYPES
            .iter()
            .map(|t| {
                AVAILABLE_TYPES
                    .get(t.pair().0)
                    .unwrap()
                    .global_defenition
                    .clone()
            })
            .filter(|x| x.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        rt.generate_functions();

        return rt;
    }

    fn generate_function(&mut self) {
        let f = CRunningFunction::new(None);
        self.code += f.finish();
        self.generated_functions.push(f.name);
    }

    fn generate_functions(&mut self) {
        for _ in 0..self.number_of_functions {
            self.generate_function();
        }

        self.code = self.code[0..self.code.len() - 2].to_string();
    }

    pub fn finish(&self) -> &str {
        return &self.code;
    }

    pub fn get_generated_function_names(&self) -> Vec<String> {
        return self.generated_functions.clone();
    }
}
