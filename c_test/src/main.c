#include "../../rustaveli/librustaveli.h"

#include <stddef.h>
#include <stdio.h>
#include <string.h>

int main() {
    struct RandomCFile* program = c_new_random_c_program(3, 1);
    char* string = (char*) c_finish_c_program(program);
    size_t n = 0;

    const char* const* function_names = c_get_generated_function_names(program, &n);
    printf("String length: %zu. '%c'\n", strlen(string), *string);
    printf("Generated %zu functions:\n", n);
    for (size_t i = 0; i < n; i++) {
        printf("    %s%s\n", function_names[i], i == n - 1 ? " " : ",");
    }

    return 0;
}