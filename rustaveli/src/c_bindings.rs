use super::RandomCFile;
use std::ffi::CString;

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
pub unsafe extern "C" fn c_finish_c_program(program: *mut RandomCFile) -> *const i8 {
    assert!(!program.is_null());
    let string = CString::new((*program).finish()).unwrap();

    string.as_ptr()
}
