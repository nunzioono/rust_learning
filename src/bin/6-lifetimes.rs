// ------------- Lifetimes -------------
//
// If a function receives a reference as parameter (mutable or not), the lifetime of the reference
// passed becomes part of the function signature.
// Is in fact needed that operations on the reference are done only while the reference has a valid
// value.
//
// Syntax:
//
// fn function_name<'a>(parameter_name: &'a type) {
//     // code
// }
//
// The compiler infers this syntax most times autonomously when the code is compiled.
// If more than one reference tho, is passed as argument can be necessary to specify if their
// lifetimes are bounded to the one which is shorter or if their lifetimes are independant.
//
// Syntax of the code inferred in the two cases:
//
// 1. fn function_name<'a>(p1: &'a type, p2: &'a type) {}
//
// 2. fn function_name<'a, 'b>(p1: &'a type, p2: &'b type) {}
//
// In the case the function operates on generics the metavariables (T,...) are specified in the
// signature after the lifetimes.
// One of the common usage of this notations in which the compiler is not able to infer the correct
// lifetimes is when the function receives two references as parameters (with different lifetimes) and returns a reference extracted by one of the two:
//
// fn function_name<'a, 'b>(p1: &'a type, p2: &'b type) -> &'a type {
//     // code
// }
//
// or
//
// fn function_name<'a, 'b>(p1: &'a type, p2: &'b type) -> &'b type {
//     // code
// }
//
// If the function saves into a local variable one of the references, the compiler is also able to
// infer the lifetime of the local variable to the lifetime of the reference.
// If the compiler is not able to do it, due to a variable derived from references with different
// lifetimes the compiler will raise an error, urges to the programmer to annotate the variable
// with the shorter lifetime.
//
// The usage of lifetimes allows both the function caller to know the lifetimes required for the
// parameters and the lifetime of the return value and to the function writer to understand when is
// a valid operation to access references. The compiler would not allow to do the opposite.
// Annotating the references will result in the compiler checking that the references are valid for
// the duration of the function and during that time that the value of the reference is not
// changed.
//
// The same is true for structs declaration.
//
// Example:
//
// struct Point {
//  x: &i32,
//  y: &i32
// }
//
// fn scale(r: &i32, p: Point) -> i32 {
//  r * (p.x * p.x + p.y * p.y)
// }
//
// !!! This will not compile !!!
// Since either the structs declarations should specify that the struct lives for the same time of
// the fields the correct version keeping the rules for functions too is:
//
struct<'a, 'b> Point {
    x: &'a i32,
    y: &'b i32
}

fn scale<'a, 'b, 'c>(r: &'c i32, p: Point<'a,'b>) -> i32 {
    r * (p.x * p.x + p.y * p.y)
}
//
// For struct methods (functions that implement self or its references as parameters) that return
// references by default
// the compiler will assume that the return value has the same lifetime as the self parameter if
// the programmer wants to return a reference that has a shorter lifetime because it comes from another
// parameter with a shorter lifetime it should specify it annotating with lifetimes.
//
// ----------------------- Closures/ Lambda's -----------------------
//
// Functional programming has introduced a concept of functions of superior order a.k.a.
// functions that have as parameter or return value a function.
// This requires that a language that wants to implement the concept of higher-order
// functions (functions of superior order) must deal with functions as if they are data types.
// In C this is achievable using pointers to functions that to the function that returns them are
// simply pointers. In C++ is possible to make so that a function can be directly assigned to
// variables.
// In Rust is possible to assign the pointer to a function to a variable or assign a value that has
// one of this traits: Fn, FnMut, FnOnce.
//
// In the first case (assigning to a variable the pointer to a function), the variable becomes
// itself a function.

fn function(i: i32, d: f64) -> f64 {
    return i as f64 * d
}

let ptr: fn(i32, f64) -> f64;
ptr = function;
ptr(10, 2.0);

// In C++ is possible to add a third kind of usage of functions as superior order.
// Is possible to allow classes to behave like functions, defining the function behaviour to the
// operator method of the class.
// Is it possible to define multiple operator methods in the class using different parameters for
// each declaration so that this can be distinguished when called.
// This implies within the operator methods member variables can be accessed, so that this
// behaviour is not really a function anymore.
// This third implementation in C++ is called functional-objects or functors.
//
// A version commonly used in various languages that tries to replicate the functional-objects
// behaviour reducing the ammount of code needed to implement it is lambda functions.
// In C++ lambda functions are defined as:
//
// [v] (int i) { return i+v; }
//
// In Rust lambda functions are defined in this way:
//
// |v| { v + 1 }
//
// parameters are defined between || separated by commas and the function code is defined within
// the curly brackets. Lambdas follow all the patterns related to an object while being essentially
// code. So they can be assigned to variables, passed or returned to and from functions.
// Also they can take ownership or borrow variables from the scope in which are defined.
// When this happens these lambda are called closures since the close within them a reusable copy
// of a value.
// When then code compiles Rust compiler maps each closure to a tuple that must implement on of the
// previously announced traits: Fn, FnMut, FnOnce.
// 
// In C++ closures can take or borrow values in one of these ways:
// - Taking the value, [v] (i) { return i+v; }
// - Borrowing the value, [&v] (i) { return i+v; }
// - Capturing values in both ways: [&v, x] (i) { return x+i+v; }
//
// In Rust closures borrow by default the values, and has to be specified with the move keyword if
// the value has to be taken.
//
// The three traits illustrated before define behaviours for the closure:
// 1. A closure that takes the value must be called only one time and must implement the FnOnce.
// 2. A closure that should update an internal status represented by a referenced value must
//    implement the FnMut.
// 3. A closure that can take or borrow values must implement the Fn.
//
// Using the closures as illustrated is possible to implement functions or high order of any type.
