#![feature(raw_dylib)]
#![feature(native_link_modifiers_verbatim)]

#[link(name = "kernel32.dll", kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
extern "stdcall" {
    fn LoadLibraryA(name: *const u8) -> isize;
}

#[test]
fn test() {
    assert_ne!(0, unsafe { LoadLibraryA("kernel32.dll\0".as_ptr()) });
}
