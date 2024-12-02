use super::RandomCFile;
use std::ffi::{c_char, CString};

#[no_mangle]
pub unsafe extern "C" fn c_new_random_c_program(
    number_of_functions: u8,
    number_of_structs_to_generate: u8,
) -> *mut RandomCFile {
    let file = Box::new(RandomCFile::new(
        number_of_functions,
        number_of_structs_to_generate,
    ));

    Box::into_raw(file)
}

#[no_mangle]
pub unsafe extern "C" fn c_finish_c_program(program: *mut RandomCFile) -> *const c_char {
    assert!(!program.is_null());
    let string = CString::new((*program).finish()).unwrap();

    string.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn c_get_generated_function_names(
    program: *mut RandomCFile,
    n: *mut usize,
) -> *const *const c_char {
    assert!(!program.is_null());
    assert!(!n.is_null());

    let function_names = (*program).get_generated_function_names();

    let c_strings: Vec<CString> = function_names
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect();

    let raw_ptrs: Vec<*const c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();

    let ptr_array = raw_ptrs.as_ptr();

    *n = raw_ptrs.len();

    std::mem::forget(c_strings);
    std::mem::forget(raw_ptrs);

    ptr_array
}
