mod functional_test;

#[no_mangle]
pub extern "C" fn main() {
    println!("main in lib.rs called");
    functional_test::test_println();
    // rust does not use implicit return code on the main, but Shadow requires return code for the plugin's main function.
    std::process::exit(0);
}