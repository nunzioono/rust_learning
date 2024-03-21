//---------------- OWNERSHIP ------------------
//
// In rust each value introduced into the program is owned by one and one only variable.
// The borrow checker, as part of the compiler, cheks effectively this assumption. So violation
// of this rule will result in a compilation error.
//
// Owning a value means being responsible of it release.
// If the value itself contains a pointer to another value it is responsible of releasing that
// resource too and signaling to the OS that it is no longer in use.
//
// The release happens when a variable goes out of scope.
// Rust offers a mechanism to control arbitrary instructions when the variable is no longer
// in use. This is implemented by the Drop trait.
// The release is demanded to another variable if its value is moved.
//
// --------------- MOVEMENT ----------------
//
// 1. When a variable gets initialized, it takes the ownership of the value.
// 2. When a variable gets assigned, it takes the ownership of the value.
// 3. When a value is passed as parameter, the function which takes the value as argument takes the ownership of the value.
//
// What happens to the variable when its value gets moved?
// If red it implies a compilation error.
// If written it succeds and enables again the variable to be red.
// When the variable goes out of scope the memory is deallocated.
//
// --------------- COPY ---------------
//
// Not all values are moved when they are assigned to different variables from the original ones.
// Basic types like numbers in Rust implement the Copy trait.
// The copy trait allows them to have the value copied instead of being moved.
// The copy trait implementation is relegated to basic types since their size and their content are
// known at compile time. Each type that contains only basic types also implements the copy trait.
// Also references to non-mutable values are copyable.
// At execution time we see no differences between copied and moved values but in the writing of
// the program we should constantly think about the difference.
//
// Appendix: Differences with C and C++
// While Rust does automatically on variable assignement goes for the movement (only for the basic
// types goes for copy), C uses always copy and C++ allows both but doesn't have any check at
// compile time so the developer has full responsibility of the memory copy or movement.
//
// ------------ DEALING WITH POINTERS WHILE CARING OF MEMORY ------------
//
// A pointer is also a value for it's variable, what about a pointer correctly allocated pointing
// to a variable that has been moved? Rust implements various pointers to allow the programmer to
// make resposibilities explicit and give rights to do things only in the right way.
//
// ------------ IMMUTABLE REFERENCES ------------
// A reference is a type of pointer that is read-only and allowes to access a variable without
// taking ownership of it. A reference can exist only while the value pointed is also existing.
// The compiler deferences the pointer automatically when the '.' (dot operator) is used.
// A reference "rents" the address of the value. While a reference holds this address is not
// possible to modify the value neither through the original variable nor through a mutable reference (that can't also be created).
// Non mutable references implement the copy trait so it's possible to create multiple references
// pointing a value at the same time leaving the ownership to the original value.
// While there is at least one reference pointing the value it can't be modified nor be
// destructed/deallocated.
//
// ------------ MUTABLE REFERENCES ------------
// Mutable references allow to modify the value pointed by the reference.
// Only one mutable reference for a value is allowed at runtime. While a mutable reference is
// allocated references cannot be created to the same value.
// Also the variable of the pointed value has to be mutable.
// 
// ------------ MEMORY REPRESENTATION OF REFERENCES --------------
// A non mutable reference is represented with the size of the address of the value.
// A mutable reference is represented with a fat-pointer (the address + the size of the allocated
// value).
// A reference to a dynamic value (a value which type is not known at compile time but is
// referenced through traits) is represented with a double pointer, it contains the memory address
// of the value and a pointer to the representation of the trait which is called vtable.
//
// ------------ LIFETIME -------------
//
// The borrow checker guarantees that all the accesses through a reference will not outlive the
// variable which owns the data (avoid the dangling pointers issues). The set of lines in which the
// reference is used is his lifetime.
// Even if in many situations the lifetime is elided from the declaration since the compiler can
// infer it from the code, it can be explicitly declared with the notation "'a" using an arbitrary
// letter. An explicit notation is useful when multiple reference to the same value are created.
// When a reference is valid for the whole duration of the program it can be annotated (even if
// it's easily inferred) with the special keyword static "'static".
// To be valid and not be signaled as an error by the borrow checker the lifetime of a reference
// should be less or equal to the lifetime of the pointed value.
// It's required to use the explicit notation when a reference is memorized inside a struct or when
// a reference is returned by a function. In the case a reference it's memorized inside a struct
// the reference has to live longer than the struct in which is memorized.
//
// ------------ SLICES -------------
//
// A slice is a reference to a sequence of values of a certain type. It's represented in memory
// with a fat pointer containing the address of the first element and the size of the slice/ number
// of the elements of the slice. A slice do not owns the values which points to.
// An extensive group of the collections is "sliceable" (Vec, String, Box, ...).
//
// ------------ ADVANTAGES OF OWNERSHIP CONCEPT -----------
//
// MEMORY ADVANTAGES
// - Nothing does not exist and cannot tho be a reference to nothing.
// - Since the validity of reference is guaranteed by the compiler, is not possible to generate
// segmentation fault or illegal access neither dangling pointers errors.
// - Since the size of types is known at each moment, is impossible to crash for underflow or
// overflow.
// - For the same reason, iterators in Rust do not exceed the number of elements.
// - All the variables are immutable, the programmer should explicitely mark them as mutable if
// wants to change their value.
// - The ownership model helps to handle logically also the ownership of restricted resources like sockets,
// file handlers, database handlers and device descriptors.
// - The absence of a garbage collector does not create non-deterministic errors.
//
// ------------ STRINGS -------------
//
// Rust offers two ways to represent strings:
// 1. As immutable arrays of charactes sequences using the
// Unicode representation for the characters using the type "str".
// Constant strings represented inside double quotes are inferred as str.
// Since the type str is not mutable it can be accessed only through slices.
// 2. Using the String type. They contain a fat pointer composed by a buffer in the heap, the size of the string and the allocated memory for the buffer (ADDRESS, SIZE, CAPACITY). The buffer gets reallocated if the memory is not enough to contain the new value assigned.
//
// &str and String are interoperable, a static string can be converted into a String and viceversa. 
//
// ------------ I/O -------------
//
// Since io operations can fail by definition and we said the nothing does not exist,
// the methods of std::io library return always the type Result<T, E>.
// To guarantee the safety of the program the programmer should test the value of a result before
// using it. The method Result::is_ok() return true if the operation succeeded.
// The method Result::unwrap() return the value if the operation succeeded and panics if not.
//
