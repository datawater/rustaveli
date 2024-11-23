# TODO: Nested structs
# TODO: Controlflow
# TODO: Interaction with the std
# TODO: Casting

import argparse
import string
import random
import functools
import operator
import copy
from sys import stderr
from typing import Any

PRIMITIVE_TYPES: set[str] = {
    "bool", "int", "char*", "float", "double", "size_t"
}

AVAILABLE_TYPES: dict[str, dict[str, Any]] = {
    "bool": {
        "size": "sizeof(bool)",
        "default": "false", 
        "random": lambda s: random.randrange(0, 1), 
        "compare": "CMP_BOOL",
        "global_defenition": "",
        "instructions": ["&", "|", "^"] 
    },

    "int": {
        "size": "sizeof(int)",
        "default": "0",     
        "random": lambda s: random.randrange(-2147483648, 2147483647), 
        "compare": "CMP_INT",
        "global_defenition": "",
        "instructions": ["+", "-", "*", "/", "%", "&", "|", "^"]
    },

    "char*": {
        "size": "sizeof(char*)",
        "default": "\"\"",  
        "random": lambda s: '"' + RandomString(amount_of_words=0, random_letters_len_min=8).value + '"',
        "compare": "CMP_STRING",
        "global_defenition": "",
        "instructions": ["append", "dup"],
        "instruction_functions": lambda a, b, i: f"__strcat((char*) {a}, (char*) {b})" if i == "append" else f"__strdup((char*) {a})"
    },

    "float": {
        "size": "sizeof(float)",
        "default": "0.0",   
        "random": lambda s: random.uniform(-65536.0, 65535.0), 
        "compare": "CMP_FLOAT",
        "global_defenition": "",
        "instructions": ["+", "-", "*", "/"]
    },

    "double": {
        "size": "sizeof(double)", 
        "default": "0.0",   
        "random": lambda s: random.uniform(-2147483648.0, 2147483647.0), 
        "compare": "CMP_FLOAT",
        "global_defenition": "",
        "instructions": ["+", "-", "*", "/"]
    },

    "size_t": {
        "size": "sizeof(size_t)", 
        "default": "0",     
        "random": lambda s: random.randrange(2147483648 * 2),
        "compare": "CMP_SIZE_T",
        "global_defenition": "",
        "instructions": ["+", "-", "*", "/", "%", "&", "|", "^"]
    },
}

class RandomString:
    WORDS: list[str] = [
        "aes",
        "decrypt",
        "encode",
        "windows",
        "startup",
        "shell",
        "network",
        "get",
        "curl",
        "decode",
        "registry",
        "update",
        "send",
        "request",
        "process",
        "spawn",
        "task",
    ]

    def __init__(self, amount_of_words: int = 2, random_letters_len_min: int = 3):
        words = [random.choice(self.WORDS) for _ in range(amount_of_words)]
        jw = '_'.join(words) + '_'

        if len(jw) == 1:
            jw = jw[:-1]

        jw += ''.join(random.choice(string.ascii_lowercase) 
                              for _ in range(int(len(jw) * 0.3) + random_letters_len_min + random.randrange(random_letters_len_min // 2)))

        self._value = jw
    
    @property
    def value(self) -> str:
        return self._value
    
class CType:
    def __init__(self, string: str):
        if string not in AVAILABLE_TYPES:
            raise TypeError(f"Invalid c type: {string}")

        self.__value = string
    
    @property
    def value(self) -> str:
        return self.__value

    def to_string(self) -> str:
        return self.value
    
    @property
    def default_value(self) -> str:
        global AVAILABLE_TYPES
        return AVAILABLE_TYPES[self.to_string()]["default"]
    
    def random_value(self) -> Any:
        global AVAILABLE_TYPES
        return AVAILABLE_TYPES[self.to_string()]["random"](self.to_string())
    
    @property
    def size(self) -> str:
        global AVAILABLE_TYPES
        return AVAILABLE_TYPES[self.to_string()]["size"]

    def valid_ctypes() -> set[str]:
        global AVAILABLE_TYPES
        return set(AVAILABLE_TYPES.keys())

class CStruct(CType):
    def __init__(self, number_of_fields: int, fields: list[CType] = [], name: str = ""):
        self.name = RandomString(1).value if name == "" else name
        self.fields = [(CType(random.choice(list(PRIMITIVE_TYPES))),
                            RandomString(amount_of_words=0, random_letters_len_min=6).value)
                       for _ in range(number_of_fields)] if fields == [] else fields
        self.number_of_fields = number_of_fields

        default_value = "{" + ",".join([x[0].default_value for x in self.fields]) + "}"
        
        def random_l(s: CStruct) -> Any:
            return functools.reduce(operator.iconcat, 
                                    [[AVAILABLE_TYPES[x[0].to_string()]["random"](x[0]) 
                                      for x in AVAILABLE_TYPES[s]["self"].fields]], [])

        AVAILABLE_TYPES[self.name] = {
            "default": default_value,
            "compare": "CMP_STRUCT",
            "size": "(" + "+".join([x[0].size for x in self.fields]) + ")",
            "random": random_l,
            "global_defenition": self.generate_defenition(),
            "self": copy.copy(self)
        }
    
    def generate_defenition(self) -> str:
        return "typedef struct {\n    " + (";\n    ".join([f"{x[0].to_string()} {x[1]}" for x in self.fields])) + f";\n}} {self.name};\n" 

    def apply_random_instruction_to_each_field(self, rhs, self_name, rhs_name) -> str:
        tr = f"(({self.name}) {{"
        
        for i in range(len(self.fields)):
            a = CVariable(type=self.fields[i][0].to_string(), name=self_name + "." + self.fields[i][1])
            b = CVariable(type=rhs.fields [i][0].to_string(), name=rhs_name  + "." + rhs.fields [i][1])

            tr += a.apply_operation(b) + ", "
            
        tr = tr[:-2] + "})"
        return tr

    @property
    def value(self) -> str:
        return self.name

class CVariable:
    def __init__(self, type: str = "", name: str = "", value: str = ""):
        self._type  = CType(random.choice(list(CType.valid_ctypes())) if type == "" else type)
        self._name  = RandomString().value if name == "" else name
        self._value = self._type.random_value() if value == "" else value
    
    def generate_defenition(self) -> str:
        return f"volatile {self._type.to_string()} {self._name} = {self._value};".replace("[", "{").replace("]", "}").replace("'", "")
        
    def apply_operation(self, rhs, i: str = "") -> str:        
        if self.type.to_string() not in PRIMITIVE_TYPES and i != "":
            print("[WARN] Structs can only have random operators applied. Returning an empty string", file=stderr)
            return ""

        if self.type.to_string() not in PRIMITIVE_TYPES:
            if self.type.to_string() != rhs.type.to_string():
                print("[WARN] Struct types don't match. Returning an empty string", file=stderr)
                return ""
        
            return AVAILABLE_TYPES[self.type.to_string()]["self"].apply_random_instruction_to_each_field(
                AVAILABLE_TYPES[rhs.type.to_string()]["self"], self.name, rhs.name
            )

        if i == "":
            n = 0

            while i not in AVAILABLE_TYPES[self.type.to_string()]["instructions"] or i not in AVAILABLE_TYPES[rhs.type.to_string()]["instructions"]:
                i = random.choice(AVAILABLE_TYPES[self.type.to_string()]["instructions"])
                n += 1

                if n > 10:
                    print("[WARN] Operation finding iteration maximum reached. Returning an empty string", file=stderr)
                    return ""

        match i:
            case "append" | "dup": 
                return AVAILABLE_TYPES["char*"]["instruction_functions"](self.name, rhs.name, i)
            case _:
                return f"(({self.type.to_string()}) ({self.name} {i} {rhs.name}))"

    @property
    def type(self) -> str:
        return self._type
    
    @property
    def name(self) -> str:
        return self._name

    @property 
    def value(self) -> str:
        return self.value 

class CUtilityFunction:
    def __init__(self, return_type = "", accept_n = -1):
        self.return_type = random.choice(list(AVAILABLE_TYPES.keys())) if return_type == "" else CType(return_type).to_string()
        self.accept_n = random.randrange(0, 5) if accept_n == -1 else accept_n
        self.accept = [CVariable() for _ in range(self.accept_n)]
        self.name = RandomString().value
        self.__code = self.__generate_defenition()
        self.__code += self.__generate_body()

    def __generate_defenition(self) -> str:
        return f"{self.return_type} {self.name}(" + ", ".join(
            [f"{x.type.to_string()} {x.name}" for x in self.accept]
        )[:-2] + ")"

    def __generate_body(self) -> str:
        tr = "{\n    "
        
        to_return_variable = CVariable(type=self.return_type, name="tr")
        tr += to_return_variable.generate_defenition() + "\n    "
        
        for _ in range(random.randrange(1, 8)):
            apply_tr = CVariable(type=self.return_type)
            tr += apply_tr.generate_defenition() + "\n    "
            tr += "tr = " + to_return_variable.apply_operation(apply_tr) + ";\n    "

        tr += "\n    "

        for _ in range(random.randrange(16)):
            op_variable = CVariable()
            tr += op_variable.generate_defenition() + "\n    "
            for _ in range(1, 4):
                apply_tr = CVariable(op_variable.type.to_string())
                tr += apply_tr.generate_defenition() + "\n    "
                tr += f"{op_variable.name} = " + op_variable.apply_operation(apply_tr) + ";\n    "
            
            tr += "\n    "

        tr += "return tr;\n"

        tr += "}"
        return tr

    def finish(self) -> str:
        return self.__code
    
class CRunningFunction:
    def __init__(self):
        self.name = RandomString().value
        self.__code = ""

        self.__op_type = CVariable()
        self.__util_functions_n = random.randrange(1, 15)
        self.__util_functions = [CUtilityFunction(self.__op_type.type.to_string()) for _ in range(self.__util_functions_n)]

        self.__code += "\n".join([util_function.finish() + "\n" for util_function in self.__util_functions])
    
        self.__code += self.__generate_defenition()
        self.__generate_body()

    def __generate_defenition(self):
        return f"__attribute__((constructor)) void {self.name}()"
    
    def __generate_body(self):
        self.__code += " {\n    "
        self.__code += self.__op_type.generate_defenition() + "\n\n    "
        for util_function in self.__util_functions:
            for accept in util_function.accept:
                self.__code += accept.generate_defenition() + "\n    "
            accepts = ", ".join([a.name for a in util_function.accept])
            result = CVariable(type=self.__op_type.type.to_string(), name=util_function.name + "_v", 
                      value=f"{util_function.name}({accepts})")
            
            self.__code += result.generate_defenition() + "\n    "
            self.__code += self.__op_type.name + " = " + self.__op_type.apply_operation(result) + ";"
            self.__code += "\n\n    "
        
        self.__code = self.__code[:-5]
        self.__code += "}\n\n"
    
    def finish(self) -> str:
        return self.__code

class RandomCFile:
    __STARTING_C_CODE: str = """// #include <math.h>
#include <string.h>
#include <stdlib.h>
#include <assert.h>
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

volatile char* __strdup(char* a) {
    size_t l = strlen(a);
    char* r = (char*) malloc(sizeof(char) * l);
    assert(r != NULL);
    memcpy(r, a, sizeof(char) * l);
    return r;
}

volatile char* __strcat(char* a, char* b) {
    size_t al = strlen(a);
    size_t bl = strlen(b);
    char* r = (char*) malloc(sizeof(char) * (al + bl));
    assert(r != NULL);
    memcpy(r, a, sizeof(char) * al);
    memcpy(r + (sizeof(char) * al), b, sizeof(char) * bl);
    return r;
}

"""

    def __init__(self, number_of_functions: int, number_of_structs_to_generate: int):
        self.__number_of_functions = number_of_functions
        self.__code = self.__STARTING_C_CODE
        self.__types_to_use = AVAILABLE_TYPES

        for _ in range(number_of_structs_to_generate):
            _ = CStruct(random.randrange(1, 12))

        self.__code += "\n".join(filter(lambda s: len(s) != 0, [self.__types_to_use[t]["global_defenition"] for t in self.__types_to_use]))
        self.generate_functions()

    def generate_function(self):
        self.__code += CRunningFunction().finish()

    def generate_functions(self):
        for _ in range(0, self.__number_of_functions):
            self.generate_function()

        self.__code = self.__code[:-2]

    def finish(self) -> str:
        return self.__code

def main():
    parser = argparse.ArgumentParser(
        prog='rustaveli',
        description='Generate random c programs',
    )

    parser.add_argument('-o', '--output', help='Output file', required=True)
    parser.add_argument('-f', '--function-count', help='Number of __attribute__((constructor)) functions to generate', required=True)
    parser.add_argument('-s', '--struct-count', help='Number of structs to generate', required=True)

    args = parser.parse_args()
    code = RandomCFile(int(args.function_count), int(args.struct_count))

    code = code.finish()

    with open(args.output, "w") as f:
        f.write(code)

    loc = len(code.split('\n'))
    print(f'[INFO] Generated {loc} LOC, {args.function_count} __attribute__((constructor)) functions and {args.struct_count} structs.')

if __name__ == "__main__":
    main()
