// -------------- Modules and tests ----------------
//
// Rust offers a package manager called cargo to manage the projects written in rust, the imported
// libraries, the test written for a given project and many other useful tools.
// A project created with cargo is called crate. A crate can be considered a group of compilable
// files. The scope of a project created in through the use of crates can be a library or an
// executable program.
// A crate that aims to be compiled as an executable is called binary crate. Usually they contain a
// main.rs file and a main() function which will be the entry point of any code in the crate.
//
// LIBRARY TYPES
//
// Each crate has a file called Cargo.toml which specifies metadata about the project, each section
// of this file is indicated by a [section] header. The Cargo.toml gets automatically created by
// cargo when a new crate is created. The [lib] section has a field called crate-type which can
// have one of the following values:
// 1. rlib
// 2. dylib
// 3. cdylib
// 4. staticlib
// The field crate-type specifies, if the crate is a library, which type of library output is
// needed.
// 1. For example specifing rlib as crate-type the compilation will result in a file that can be
// included only by other rust files.
// 2. A dylib will result in a .dll file if on
// windows, a .so file if on Linux or .dylib on MacOS. This kind of library is called dynamic since
// it can be loaded only a runtime. It can be used only by rust programs.
// 3. A cdylib will result in a .dll file if on windows, a .so file if on Linux or .dylib on MacOS
// but will be compiled using the C abi standard so that the result can be loaded by any
// language.
// 4. A staticlib will result in a static library compiled as .lib on windows or .a in Linux and
// MacOs. This kind of library is included in a project at compile time and can be used by any
// language. It requires to be imported as an external function if used in a Rust program.
//
// MODULES
//
// A crate is represented internally as a tree of modules. A module can be a single file with .rs
// extension or a folder containing a mod.rs file.
// A single file can contain multiple modules declaring them explicitely as the code contained
// within the declaration "mod module_name {...}". Each element within a module is private (cannot
// be imported outside of the module) if not marked with the pub keyword.
// A module declaration preceeded by the pub keyword is accessible also if is parent module is
// imported.
// To import modules declared in same file of the entry point of a crate is possible to call their
// elements using the crate keyword followed by the "::module_name". If the module is declared as a
// file can be imported using the "use" keyword. Note that if the module path is declared starting
// from the current module it should be preceeded by "self::" or if relative to the module parent
// "super::". 
// 
// DEPENDENCIES
//
// Crate dependencies can be declared under the section [dependencies] in the Cargo.toml file.
// They should be declared as the name of the external crate followed by = "version number".
//
// PRELUDE
//
// The prelude module contains essential rust types like Vec, String and so on.
// The prelude module is always included in the Rust project by default. Cargo decides which
// version to include depending on value of package/edition field in Cargo.toml.
//
// TESTS
//
// A test in conceptually a set of line of code that uses a certain element of the main code to
// check if it works as intended. Tests are important if written in a balanced way before and after
// the actual implementation of the code. Piloting the program to act as the tests written before
// want and not breaking features with the tests written after.
// Tests on the behaviour of a single element of a module are called unit tests.
// Tests on the behaviour of the interaction between two modules are called integration tests.
// Tests on the behaviour of the whole program are called end-to-end tests.
// The whole stack of tests consists of a manual.
// 
// The submodule in which the unit-tests are written is commonly called tests.
// The submodule of unit tests has to be preceeded by the #[cfg(test)] annotation.
// Using the command `cargo test` the tests will be executed.
// The integration tests instead are usually written inside of a "tests" folder in the same
// directory of src folder.
// To test the behaviour of modules the integration tests have to import the module using the
// keyword "use" followed by "crate::module_name".
// A test is a function marked with the #[test] annotation.
// The implementation of a test usually prepares the variables on which the code will be executed,
// execute the code needed to simulate a functionality and then checks if the result is as expected.
// To check the result of the code the std library provides the assert!, assert_eq! and assert_ne! macro.
// To verify an error is excepted is possible to annotate a test with the #[should_panic(expected="message")] annotation.
// If a test returns a Result<T,Error> is possible to use the condition to check if test is failed.
// Is possible to execute tests containing a specific substring in the name using the command
// "cargo test" followed by the substring.
// Is also possible to annotate tests with the #[ignore] annotation to ignore them when using cargo
// test.
// Also ignore annotated test can be executed using the command "cargo test -- --ignored".
//
//
//
