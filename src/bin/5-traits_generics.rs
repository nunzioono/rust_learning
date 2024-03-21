// ---------------- Traits ----------------
// Often in programming is needed to minimize the code written or the size of a program.
// Many languages attempt to give ways to programmers to implement common functions that can be
// reused even changing a minimal portion of the code itself.
// Usually languages like Java, Javascript and others use interfaces, which are constructs that
// contain set of methods, the classes can then implement these interfaces and use them in the
// program (one or more methods can be overridden). Also classes can extend other classes. This behaviours are called respectively inheritance and polymorphism.
// Languages like C++ can implement polymorphism using virtual functions and virtual tables and
// allow to extend classes multiple times.
//
// Rust provides to a assolve this needing providing traits. The traits are a set of methods.
// They can be used in multiple classes. They can be overridden and derived from the parent class
// (inheritance). Differently from C++ they do not imply normally an additional memory cost.
// This additional cost kicks in when an object is referenced by his trait and his type has to be
// determined at runtime.
//
// --------------- Traits syntax ----------------
// 
// To define a trait is necessary to use the following syntax:
//
// trait Name {
//     method1();
//     method2();
//     method3();
//     ...
// }
//
// The return type of a method is mandatory, otherwise it will be assumed is ().
// When the keyword Self is used as parameter or return value the type is inferred as the one of
// the data structure that implements the trait.
// As in data structures implementations, if the parameter self is not set within the function
// within the trait, that function will be static and so it will be available without instances of
// the trait.
// Example:
//
// trait Name {
//     method1();  
// }
//
// fn main() {
//   Name::method1();   
// }
// 
// Rust allows to use type aliases to implement traits. Their usage is limited to the code readability.
// 
// Example:
//
// trait Name {
//     type NumberWithSign = i32;
//     method1(arg: NumberWithSign);
// }
//
// Since traits are pretty popular in rust it is possible to make traits that implement already
// existing traits, avoiding so to write already existing code.
// The trait that implements another trait is called subtrait while the implemented trait is the
// supertrait. Implementing a subtrait on a data structure requires also to implement the
// supertrait.
//
// -------------- Traits implementation ----------------
//
// To make so that a data structure like a struct or an enum implements the methods of the trait
// there are 2 approaches:
//
// 1. use the keyword impl:
//
// impl Name for Type {
//     method1() {
//         // do something if want to override
//     }    
//
//     method2(); // do nothing if want to inherit the method as it is
//
//     // note that not specifing a function it gets the default implementation
//     // method3() is imported as declared in the trait
// }
//
// 2. use the attribute #[derive] if the data structure has all the fields in it already
//    implementing the trait:
// 
// #[derive(Name)]
// struct Type {
//     ...
// }
// 
// If the type of a data structure that implements a given trait is known by the compiler, the
// trait methods will be used without any time penalty, this is known as static invocation.
// It is also possible to reference an object by the trait it implements, this is known as dynamic
// invocation.
// For example inside a function specify that one of the arguments has not a specific type but
// implements a known trait:
//
// fn method(arg: &dyn Name) {
//     // do something
// }
//
// The references to a trait are commonly called object-trait.
//
// ------------- Standard traits ----------------
// 
// Rust has many standard traits that are already implemented in primitive types and as seen 
// can be implemented as supertraits to our new traits. So it is useful to know a bunch of the
// common used traits of the standard library:
//
// - Eq, PartialEq, Ord, PartialOrd: to compare values with operators like ==, !=, <, >, <=, >=
// - Add, Sub, Mul, Div, Rem, BitAnd, BitXor, Shl, Shr, Neg, Not: respectively allow a type to add, subtract, multiply, divide, remainder, bitwise, shift operations, invert the sign, make the logical not of the bits.
// - Display, Debug: to convert a type into a string
// - Copy, Clone: to make a type copyable
// - Drop: to make a type droppable, is esclusive on types with the Copy trait.
// - Index, IndexMut: to index a data structure as an array.
// - Deref, DerefMut: to access a value from his pointer.
// - RangeBounds: to iterate over a range of values of the type with a syntax like
// ..5,1..,1..5,1..=5.
// - From, Into: allow to cast the type, when one of the two traits is implemented the other gets
// inherited automatically.
// - TryFrom, TryInto: same as From and Into but return a Result since the operation can fail.
// - FromStr: derives the tryfrom and allows to cast a string to the type that implements it,
// returns a result since can fail.
// - Error: is a trait that is used to perform error logging when a result contains a failure of an
// operation. To pass a custom error when our code fails we need to implement it as supertrait.
//
// ---------------- Generics ----------------
//
// Types allow robustness of the code defining which operations are possible and which not.
// Since each type can have a different set of operations allowed this will need a lot of code.
// Rust allows to define generic types to reduce the amount of code that need to be written for
// defining an operation for multiple types. This mechanism is called generics.
//
// The signature of a function that operates on generics contains a letter that will indicate the
// generic type.
//
// "fn add<T>(arg1: T, arg2: T) -> T"
//
// To define a constraint over the generic type we coherce it to implement a specific trait:
//
// "
//   fn max<T: PartialOrd>(arg1: T, arg2: T) -> T {
//     if arg1 > arg2 {
//         arg1
//     } else {
//         arg2
//     }
// "
//
// This can be done in the same way to include generics within data structures:
//
// struct MyStruct<T> where T: Display {
//  foo: T
// }
// 
// ---- Note that MyStruct<T: Display> is the same as MyStruct<T> where T: Display ----
//
// impl<T> struct MyStruct<T> where T: Display {
//  print(&self) {
//      println!("{}", self.foo);
//  }
// }
//
// ----------------- Summary for Traits and Generics ----------------
//
// Traits define a set of methods for a type.
// Generics define a type 0...all of his traits.
// Generics shouldn't be used if not for defining a method that should exist on many types or a
// function that takes arguments or returns values of many types.
// Generics when used in this way bring efficiency to the compiler, since they allow to define
// methods without moving on a VTable of a trait.
// Not all the traits allow the definition of object-traits, it's allowed to use dynamic invocation
// of traits in methods only if the trait doesn't contain any static function (without self, &self,
// &mut self).
// But it is possible to define an object-trait that implements multiple traits (they do not have
// to be subtraits of each other tho).
