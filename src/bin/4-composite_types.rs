// ------------ Composite types -------------
//
// In C and C++ the struct keyword allows to create new types that contain group
// of fields, each one opened to external usage. In C++ is possible to associate methods
// to an instance of the struct or to a specific type. In C++ is also possible to create classes,
// which are composed by different fields as the struct but classes allow to modify the visibility
// of each field specifying if the fields are usable only by instances of the same class
// (private), by instances of the same class and its subclasses (protected) or if every other
// portion of code can access class fields (public), the same is valid for class methods.
//
// Another feature of C and C++ regarding composite types is that in each one of these languages is
// possible to define additional structures called enum and union.
// The enum type defines a set of constants that get associated with integers. From C++11 onwards
// is possible to associate the constants with other scalar types like char, short, etc...
// The union type defines a group of fields that will be either represented by the union type.
// The union type takes an amount of memory that is equal to the largest field type.
//
// ------------ Composite types in Rust ------------
//
// As his ancestors Rust allows to represent composite types defining new composite types using
// keywords like struct, union and enum.
//
// ------------ Struct declaratinos in Rust -------------
//
// A struct is a collection of eterogeneous values as fields, to create a new struct the name of
// the struct, the name of
// the fields and their types have to be specified.
// For convention struct names are written in CamelCase.
// For example the declaration of the Point struct is:
// struct Point { x: i32, y: i32 }
//
// His instances are generated in either one of these ways:
//
// 1. Initializing each field: "let point1 = Point { x: 0, y: 0 };"
// 2. Using fields from another struct instance of the same type: "let point2 = Point { x: 1, .. point1 };"
// This method creates a new instance that has as fields the same values of the previously passed
// struct and then ovverrides the fields with the values passed in the constructor. Note that is
// also possible to create new instances not passing any value but an instance of the struct of the same type.
// 3. Using a defined function called constructor:
// impl Point {
//     fn new(x: i32, y: i32) -> Point {
//          println!("This is a custom constructor!");
//          Point { x, y }
//     }    
// }
// 
// The struct values are accessed from the instance using the dot operator:
// "let instance = Point { x: 1, y: 2 };
// instance.x = 2;
// assert_eq!(instance.x, 2);"
//
// Is also possible to create structs without using names for fields using a declaration of this
// type:
//
// "struct Point (i32,i32);"
//
// In this way the rappresentation in memory is similar a tuple and the fields can be accessed with
// the dot operator followed by the index of the order in which the fields are declared, with the exception 
// that in tuple is not possible to define methods for a tuple. Is also possible to define empty
// struct in this way:
//
// "struct Empty ();"
// 
// Both struct and struct fields visibility can be set to public or private, the default value is private and
// prefixing a field name with pub it changes to public visibility.
// This mechanism allows to make the struct hidden to outside of its module and its submodules.
// A struct that has private visibility to be useful tho should have methods that are accessible
// publically.
//
// ------------- Struct representation in memory in Rust --------------
//
// The representation of structs in memory is not predictable since the compiler operates on the
// allocatable memory to align fields size to a power of 2.
// Using constructs of the language is possible to align manually the fields specifying a padding
// (a number of bytes) that have to be interposed between the fields.
// In particular is possible to impose the same representation in memory of the padding between the
// fields that C uses. This is done specifying an attribute before the struct declaration:
//
// #[repr(C)]
// struct Person { name: String, age: u8 }
//
// The size of a value or of an instance of a struct can be obtained with function "std::mem::size_of_val()"
// also the alignment can be known with "std::mem::align_of_val()".
// 
// -------------- Struct methods in Rust ----------------
// 
// In Rust it is possible to define methods for structs using the impl keyword followed by
// curly parenthesis.
// Within the block defined by the parenthesis the methods are defined as ordinary functions with
// the exception of the possibility to include between their parameters the self parameter.
// The self parameter is used to access the struct fields.
// Functions that use the self parameter or pointers to it are called methods.
// Following the ownership rules a function that wants to take ownership of the struct itself uses
// the "raw" self parameter (very rare since taking ownership within a struct method means inhibit the usage of struct to methods that are called after this method call) while methods that return metadata based on the struct fields, fields values or reference to the fields values use the "self" parameter with an immutable reference to the struct a.k.a. &self.
// Methods that want to modify the fields instead take as parameter a mutable reference to the struct
// a.k.a. &mut self.
//  
// Examples:
//
// "struct Point {
//  x: i32,
//  y: i32,
// }
// 
// impl Point {
// fn mirror(self) -> Self { // Consumes the struct Point itself and returns a new struct of the
// same type and with the same values
//  Self{ x: self.y, y: self.x }
// }
//
// fn length(&self) -> i32 { // Returns a value computed on the struct fields
//  (self.x*self.x + self.y*self.y).sqrt(self.x*self.x + self.y*self.y)
// }
// 
// fn scale(&mut self, s: i32) { // Modifies the struct fields
//  self.x *= s;
//  self.y *= s;
// }
// }
//
// let p = Point { x: 3, y: 4 };
// let p2 = p.mirror();
// assert_eq!(p2.x, 4);
// assert_eq!(p2.y, 3);
// 
// let p3 = p.length();
// assert_eq!(p3, 5);
// 
// let mut p4 = Point { x: 3, y: 4 };
// p4.scale(2);
// assert_eq!(p4.x, 6);
// assert_eq!(p4.y, 8);
// "
//
// In Rust a concept different from C++ is the absence of constructors, even if for convention a
// function new() is defined within the struct implementation, in Rust constructors by default
// are not defined. The construction of the struct is not relegated to the implementation of
// the struct, it can be done outside of it and it can be done by as many functions as needed.
// Things as the multiple constructors in C++ are realized using simple functions with different
// names.
//
// Examples:
//
// "pub fn new() -> Self {...}
//  pub fn with_details(...) -> Self {...}"
//
// Same as constructors things as the destructors in C++ are not implemented in the same way in
// Rust, they are not defined in the struct implementation. In Rust when an instance of a variable
// (which can be a struct) goes out of scope, the std::mem::drop method is invoked to release the
// memory. A way in which Rust allows to customize the operation to execute on destruction is to 
// add a Drop trait to the struct and define the drop method.
//
// Examples:
//
// "struct Person {
//   name: String,
//   age: u8,
// }
//
// impl Drop for Point {
//   fn drop(&mut self) {
//     println!("Dropping {}", self.name);
//     std::mem::drop(self);
//   }
// }"
//
// A constraint that is applied to this Drop trait implementation is that is possible to
// implement it only if the fields of the struct are not all primitive types / the struct cannot
// already implement the Copy trait. This comes as simplification of the code and also a security feature.
// This implies drop methods cannot be redefined for primitive types.
// 
// In C++ is possible to define static methods in classes, which are functions that can be called
// outside of an instance of the class itself. In Rust the analog functionality is realized on each
// function inside the impl block that does not have the self parameter.
//
// --------------------- Enums in Rust --------------------
//
// In Rust enums are more powerful of C and C++ since they allow to bind each constant not only
// with primitive types but also composite types, furthermore for enums can be implemented methods
// as for structs.
// Rust provides a useful construct called match that can be used to match an enum value to execute
// an arbitrary block of code.
// The memory that enum takes is equal to the largest size of the enum values associated by constants.
// On default enum constants are mapped to u8 numbers.
//
// --------------------- Unions in Rust ---------------------
// 
// Unions in Rust are used to represent data types that have optional values.
// Their representation in memory is equal to the largest field type size.
// Unions can have methods in the same way enums do.
// Litterally unions have same utility of enums while enum provide modern and better features.
