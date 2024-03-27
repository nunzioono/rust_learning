// --------------- Smart pointers -------------------
//
// Each value assigned in a program is memorized in the address range of memory of a process.
// Usually in Rust and C++ references allow to get the memory address of the first byte in which
// values gets memorized.
// In Rust using the reference in a program activates the borrow checker to start checking bounds
// to the variables to keep the safety guarantees of Rust.
// The dual operation of getting a value from a reference is called dereferencing.
// A dereference operation is indicated using * before the name of the reference variable or using
// the . operator.
//
// The type system of Rust and C++ allows to create types with generic purposes so in the time the
// composition of the generic types and the deferencing operator provided a popular solution in
// which some types are created to be then dereferenced making guarantees on the pointed data.
// This concept is often called "smart pointers".
// 
// Due to the strict rules on ownership imposed by Rust, is impossible to create circular
// structures using only references. The only way without explicitly using unsafe code is to use
// smart pointers like Rc or Arc (if in multi-threaded environment).
//
// ----------------- Smart pointers in C++ -----------------
//
// In C++ is possible to declare smart pointers like unique_ptr or shared_ptr.
// The first created by the function make_unique passing the original value ensures that only a
// pointer will exist to the original value and if one more gets created it will create an error.
// Shared prt is created with make_shared passing it the value, it keeps a counter of the
// references pointing a value and when it reaches zero it allows to deallocate the original value.
// Since this mechanism a circular structure cannot be created using unique or shared pointers. In
// this case is possible to use weak_ptr which can be instantiated using a downgrade option on a
// shared pointer.
//
// ----------------- Smart pointers in Rust -----------------
//
// In Rust a major variety of smart pointers exists.
// Many pointers can be mapped to the same functionalities that C++ provides like:
// - The Box pointer in rust is similar to the unique_ptr in C++
// - Rc is similar to shared_ptr in C++
// - Weak is similar to weak_ptr in C++
// Tho the variety in Rust is also given by the differentiation from pointers that have to be used
// in non threaded environments and in threaded. Like Rc in Rust as a version of the same
// functionalities to be used in multi-threaded environments called Arc.
// And some other pointers in Rust refer to abstractions that are only related to how Rust works,
// like Cell, RefCell and Cow are pointers that allow to bypass rules imposed by the borrow checker
// at compile time. And Mutex and RwLock are pointers that can be used in multi-threaded
// environments to maintain consistency between threads.
// In general smart pointers consists of generics with metadata fields used to the specific
// functionality they implement and the implementation of the traits Deref and DerefMut.
//
// ---------------- Box -----------------
//
// Box allows as smart pointer to take ownership of the data in memory. Allowing so to move this
// pointer between functions without the need of expliciting the life time of the data since the
// box pointer will guarantee his ownership.
// Box is also used to contain pointer to dynamic values, like values which have not known type at
// compile time. This is done pointing the trait vtable they implement rather than the data itself.
//
// ---------------- Rc ----------------
//
// Rc is a smart pointer which contains a reference to the value and two more numerical fields
// which represent the actual references to the same data in the program.
// They get incremented when a new reference is created using and decrement when a
// reference goes out of scope. The first counter is incremented when Rc::clone() is called.
// The first counter in Rc counts the number of effective copies of the value and is called strong
// counter. While the second counts only the weak references to it and is then called weak counter.
// While multiple references can be created using & the Rc smart pointer will take a copy of the data
// ignoring the borrow checker rules and allowing than the programmer to ebsentially do nothing
// else than a copy instead of a reference.
// Alone will not give much of a real usage but when combined with RefCell it can be used to create
// multiple mutable references to the same data.
//
// Note that the increment of strong and weak counter is not thread safe but gets significant
// performance gains. To use a thread safe version of Rc is possible using Arc.
// 
// ---------------- Weak ----------------
//
// The weak pointer does the same thing of Rc but allowing only a counter on the real references to
// the data and not to the copies. This allows the programmer to write circular structures like
// trees and graphs.
// A weak value can be only created using Rc::downgrade() and can return to an Rc using Weak::upgrade().
// The upgrade operation results in an Option value that will be None if the original value is
// nomore in memory or if the Rc is still in memory.
//
// ---------------- Cell ----------------
//
// The cell pointer is used for internal mutability, it allows to modify a field inside of an
// immutable structure. It substancially avoids the compiler to check the pointed data creating a
// copy on the cell creation and then using mutable pointers to change the data.
// Cell provides the following methods:
// - get() to get a copy of the contained value
// - take() to get a the contained value moving it outside of the cell and replacing it with the
// default for that type (requires Default trait)
// - replace(value) to replace the contained value with a new one
// - into_inner() to return the contained value and the dropping the cell
//
// ---------------- RefCell ----------------
//
// Given that Cell does use references to the data to change it, externally it does not allows the
// programmer to point the data. Here comes RefCell which allows to do this.
// RefCell implements method like borrow() and borrow_mut() to get a reference to the data.
//
// ---------------- Cow ----------------
//
// Cow is a smart pointer that allows to create a pointer to a value and when the value gets
// modified to update immediately the references to the value. Cow infact stands for Copy On Write.
// Is implemented as an enum and can generate an instance of the pointer using the Cow::from()
// method.
// If the value is copiable the Cow pointer will contain an Owned instance and if is not copiable a
// Borrowed instance. When Cow::clone() is called it will return another pointer containing a copy of the value.
//
// -------------- Additional notes ----------------
//
// The self parameter in data structures can also have Box<Self>, Rc<Self> or Arc<Self> type but
// must be explicitly used.
// counter only works for weak references.
