#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RandomCFile RandomCFile;

struct RandomCFile *c_new_random_c_program(uint8_t number_of_functions,
                                           uint8_t number_of_structs_to_generate);

const char *c_finish_c_program(struct RandomCFile *program);

const char *const *c_get_generated_function_names(struct RandomCFile *program, uintptr_t *n);
