// --------------- VARIABLES AND TYPES -----------------
//
// A variable binds a name to a value, in Rust this bind is created using the 'let' keyword.
// Rust favours immutability so when a variable is created it is automatically set to be immutable.
// To allow mutability during the program use the 'mut' keyword.
// Each variable when initialized has a type, if not specified a type gets inferred from the value.
// The type coherces the successive instruction to set the variable value only on certains values
// allowed for the type, also it coherces the programmer on the operations that can be performed on
// the type.
// A type can be also explicitely declared in the declaration of the variable using the ':'
// operator after the name of the variable and the type.
//
// ---------------- VALUES ---------------
//
// The value of a variable can be specified or produced evaluating an expression.
// An expression gets evaluated by default moving from left to right, this order can be modified
// using the round parententhesis.
// In Rust each expression produces a value, even when the expression doesn't logically return
// anything it returns an empty tuple '()'.
// 
// ---------------- TYPES AND TRAITS ----------------
//
// Rust provides a set of predefined types that can be used in the program.
// These types are called elementary or primitive types:
//
// - tuple
// - strings
// - arrays
// - slices
// - never
// - pointers
// 
// and so on...
// Differently from other languages types are not organized in a hierarchy.
// It's inevitable that 2 or more types share properties so Rust provides a mechanism to represent
// these relationships using traits.
// A trait describes a set of methods that a certain type can implement.
// The compiler uses the information about the trait to evaluate the correctness of the program at
// compile time.
// Traits are similar to what in the other languages is done with interfaces.
//
// As said before inevitably unrelated primitive types share some traits for example:
//
// i32 implements the 'Copy' trait.
// Vec and String share the 'Clone' and 'Drop' trait.
// i32, Vec and String implement the 'Send' trait.
// i32 and String share the 'Display' trait.
// And many other types and traits have methods that come from the same traits even if the types
// are not related directly.
//
// ----------------- PRIMITIVE TYPES ----------------
//
// Primitive types in Rust are:
//
// - Numbers with sign: i8, i16, i32, i64, i128, isize
// - Numbers without sign: u8, u16, u32, u64, u128, usize
// - Numbers with floating point: f32, f64
// - Boolean: bool
// - Character: char (they are encoded using unicode standard)
// - Unit: (), it represents as said above the empty value.
//
// ---------------- STRINGS ----------------
//
// Strings are implemented in Rust using more than one type to allow programmers versatility on
// implementation of low level programs:
// 
// - &str, is a primitive type that is a slice of bytes that are valid in the UTF-8 encoding.
// - String, is a type that collects bytes in a Vec. String type is allocated automatically on the
// heap.
//
// ---------------- TUPLES ----------------
//
// Tuples are ordered collections of eterogenous values. Tuples are constructed using the '()'
// operator.
// Each field of a tuple can be accessed using the 'dot' operator followed by the index of the
// value.
//
// ---------------- REFERENCES, BOXES AND RAW POINTERS ----------------
//
// Rust offers different ways to represent memory addresses:
//
// 1. References that can be immutable or mutable and are represented by '&' and '&mut'.
// 2. Box that is represented by a type itself.
// 3. Raw pointers, can be immutable and mutable and are represented by '*const' and '*mut'.
//
// Differently from C and C++, the usage of pointers is "simplified" by ensurance made by the compiler.
// In particular the compiler checks if a pointer has possession of the data and if a pointer is
// valid verifying that the data is accessed while his lifetime is valid.
// In rare cases the programmer wants to elude this checks and can do it enclosing the code within
// an unsafe block.
//
// A mainly difference between references and pointers is that references cannot be null.
// Neither references can refer an expired value.
// A special system within the compiler called BorrowChecker is in charge of verifying the above
// conditions and restricting in a given scope that will always be only one writer or multiple
// readers.
// C++ have references but they are mainly different from Rust ones.
//
// All local variables are allocated on the stack, when we want to create a variable that survives
// the scope in which is declared we should use the Box type that forces the specified variable to
// be allocated on the heap instead.
// The content of the Box tho is accessed in the same way as the references and raw pointers do,
// using *name_of_the_variable notation also called dereferencing the pointer.
// 
// EXAMPLE OF USAGE OF BOX
//
// fn makeBox(a: i32) -> Box<(i32,i32)> {
//  let b = Box::new( (a, 1) );
//  return b;
// }
//
// fn main() {
//  let b = makeBox(5);
//  let c = b.0 + b.1;
// }
//
// The dereference of a raw pointer requires to be done within an unsafe block.
//
// ---------------- ARRAYS -----------------------
//
// Arrays are ordered collections of omogenous values. Arrays are constructed using the '[]'
// operator. If an array is declared it MUST have a known size and this have to be specified
// during the declaration of the array itself.
//
// EXAMPLE OF UNINITIALIZED ARRAY
// let array: [i32; 10];
//
// EXAMPLE OF INITIALIZED ARRAY
// let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//
// EXAMPLE OF INITIALIZED ARRAY WITH FIXED VALUE
// let array = [3; 10];
//
// EXAMPLE OF ACCESS TO A VALUE WITHIN AN ARRAY
// println!("{}",array[0]);
//
// --------------- SLICES -----------------------
//
// Slices are references to a sequence of elements contained within an array.
// They are implemented using the address of the first element and the length of the sequence.
// This couple of informations is also called 'fat pointer'.
// Basically a slice is immutable, it should be declared otherwise specifying &mut and the array
// portion to be sliced.
// To access an element within a slice the same method of the arrays is used.
//
// --------------- VEC ----------------
//
// Vec is a dynamic array (dynamically sized = can grow) that is allocated on the heap.
// The Vec type is internally implemented as a tuple of 3 values:
//
// - pointer to a buffer on the heap
// - length of the buffer
// - elements allocated in the buffer
//
// When the program asks to add an element to the Vec, the vec checks if the buffer is full and if
// it is allocates more space for the buffer to allow the addition of the new element.
// NOTE: that the previous buffer is first copied into the new buffer and then DEALLOCATED, 
// so the buffer has different addresses in memory before and after an insertion that triggers
// the reallocation.
//
// --------------- FUNCTIONS ----------------
//
// Functions are the core element of any program.
// They are defined using the 'fn' keyword.
// Then should be specified a name for the function,
// a list of parameters
// and a return type (when not specified it's '()').
// The compiler also checks if any value is returned inside the body
// checking if it's not followed by ";".
//
// --------------- INSTRUCTIONS AND EXPRESSIONS ----------------
//
// An istruction is an action that is separed by the others using ";" at the end of a line.
// An expression instead returns always a value and is characterized by a block "{ }" with a series
// of instructions and a return value.
// IF, ELSE and LOOPS are considered as expressions.
// An expression can be interrupted using special keywords as "break" and "continue".
// Expressions can be annidated one within the other.
// Between the loops FOR has a significative difference from classic programming languages.
// As expression has to return a value or a set of values that can be built into an iterator.
// Iterators compatible values are for example: arrays, slices and range.
// 
// Another add-on to classic syntax is the usage of ranges as collections of consecutive comparable elements,
// for example '1..5' is used to resamble the range of numbers from 1 to 4.
// A symbol '=' can preceed the last value in a range to include in the range the last value.
// Example: '1..=5' is used to resamble the range of numbers from 1 to 5.
//
// Another add-on to the classic syntax is the statement MATCH. It is used to execute conditionally
// a block of code specifying a pattern to test against a specified value. Each pattern is
// specified as "PATTERN => { CODE TO BE EXECUTED IF THE PATTERN IS TRUE },"
// To used the same name of the variable given in the pattern inside the corresponding block of code
// the name of the variable within the pattern can be preceeded by the '@' operator.
//
// If the pattern contains only an identifier name within the pattern itself, the variable passed to the match expression automatically
// is assigned to that name.
// If the pattern contains no names the value can be compared against special operators as:
// - '..=' that means the value is included in the range
// - '|' that means the value is either equal to the first element OR the second.
// - '_' that means if no other pattern got matched before this one will match.
// The order in which the patterns are written is the same in which they are evaluated and if one
// matches the followings patterns are ignored.
//
// EXAMPLE OF SPECIAL OPERATORS WITHIN A MATCH EXPRESSION
//
// let values = [1, 2, 3];
// match &values[..] { // crea una slice con tutti gli elementi
//   Contiene almeno un elemento, il primo valore è 0
//   &[0, ..] => println!("Comincia con 0"),
//   Contiene almeno un elemento, l’ultimo valore è compreso tra 3 e 5
//   &[.., v @ 3..=5] => println!("Finisce con {}", v),
//   Contiene almeno due elementi
//   &[_, v, ..] => println!("Il secondo valore è {}", v),
//   Contiene un solo elemento
//   &[v] => println!("Ha un solo elemento: {}", v),
//   Non contiene elementi
//   &[] => println!("E' vuoto")
// }
//
// ---------------- RUST COMMUNITY NAMING CONVENTIONS ------------------
//
// Rust community has shared a common sense rules for naming conventions.
// They are:
//
// - Names of struct, enum, traits and other predefined rust keywords should be in UpperCamelCase.
// - Names of variables, functions, methods and other non-predefined rust keywords should be in
//   lower_snake_case.
