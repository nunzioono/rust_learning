// Allocation
//
// ----------------- PROCESSES AND MEMORY ------------------
//
// Each process when created gets a chuck of memory. Each chunk of memory assigned by the operative
// system has a collection of metadata indicating the allowed operations on that memory:
//   - read
//   - write
//   - execute
//   - copy_on_write
//
// If the code of the process tries to utilize the memory in a not-allowed way it provokes and
// interruption of the process that in windows is called ACCESS VIOLATION and in unix systems is called
// SEGMENTATION FAULT.
//
// The chunk of memory assigned to the process gets divided in different "zones" having each one a
// different usage.
// The zones of memory in chunk assigned to the process are:
//
// - Executable code, contains the code to be executed to assolve the process
// - Constants, contains immutable variables and can be only red
// - Global variables, contains variables that can be both red or written
// - Stack, contains local variables, arguments of the program, return values and the address that
// have to be used to return the execution to the caller of the process
// - Heap, contains memory that can be accessed only using pointers, can be both red or written
//
// An example of the stack in a program containing a simple function that returns the product of 
// the argument passed plus one multiplied for the argument passed plus one:
//
// fn f(a: i32) -> i32 {
//  let b = a + 1;
//  return b * b;
// }
//
// fn main() {
//  let x = f(2);
//  return x;
// }
//
// This is the stack in this example:
//
// ----------------------
// | res: |           | |
// |      ------------- |
// | return address     | 
// | ------------------ |
// | x: |           |   |
// |      ----------    |
// ______________________
// | res: | 9        |  |
// |       __________   |
// | a:   | 2        |  |
// |       __________   |
// | return address     |
// | ------------------ |
// | b: | 3        |    |
// |      ----------    |
// ----------------------
//
//  ! Note that it grows to the bottom !
//
//  -------------- LANGUAGES API FOR MEMORY ALLOCATION ----------------
//
//  In C language variables are distinguished as global, local and dynamic.
//  Each type has a different life cycle.
//
//  - Global variables have an static address, determined by the compiler and the linker
//  At the start of the program are initialized.
//  - Local variables have a logical address depending on the scope of the variable (the function
//  that contains it) as seen in the previous example.
//  Also their life cycle is equal to the scope in which they are declared.
//  At the boot of the program they contain a random value.
//  - Dynamic variables have an absolute address determined in execution of the program,
//  they are accessible only using pointers, the programmer determines their lifetime and the value
//  can be initialized or not.
//
//  These mechanisms can be used only if the language (as C does) provide a way to allocate and
//  deallocate memory inside of the chuck given by the OS.
//
//  C uses malloc, calloc, realloc and free.
//  C++ uses new, new[], delete and delete[].
//  
//  Each one of these operations operates on pointers.
//  
//  ----------------- POINTERS ------------------------
//
//  Pointers are always a number referring a specific location on the memomy, so they are in range
//  0..2^n - 1.
//  They can be INVALID tho, they can contain a number that represents a location that the
//  OPERATIVE SYSTEM not assigned to the process.
//  Also for programmers is sometime useful represent invalid pointers in some way,
//  various representation of the invalid pointer are available:
//  - 0
//  - NULL or (void*) 0
//  - nullptr (used in C++11 and successive versions)
//  When using a pointer is our responsibility to establish which permissions are we allowed to
//  take using it.
//  These freedom comes with potential questions:
//
//  - Is it valid?
//  - How much memory am i allowed to access from the address of the pointer?
//  - How big is the content of the pointer?
//  - Can i write at the pointer address?
//  - Do i need to release it?
//  - Do any other code scopes contain the address of the pointer?
//  - Is it only a symbol representing if the data is valid?
//
//  The usage of pointers is usually divided in categories:
//
//  CASES IN WHICH THE POINTER DOES NOT OWN THE DATA
//  - Used to access data at the moment that is passed by someone else 
//  - Used as a way to pass the address in which we want a function writes in
//  - Used to access consecutive data in memory as arrays, !!! they can grow over the allocated
//  memory !!!
//
//  CASES IN WHICH THE POINTER OWNS THE DATA (SO HAS TO DEALLOCATE THE MEMORY WHEN DONE WITH IT)
//  - Used to access a data in the heap
//  - Used to represent an optional return value from a function
//  - Used to implement complex data structures
//
//  The usage of pointers in the case of owning data so can cause serious problems.
//  C++ uses smart pointers that automatically deallocate the memory.
//  So the programmer has less to think about memory while realizing a program.
//
//  Potential problems when not using a good strategy for allocation and deallocation of pointers
//  are:
//  - Dangling pointers, usage of pointers that have been deallocated. Produces undefined behavior
//  since the memory pointed can contain any value.
//  - Memory leakage, memory cannot be used anymore since the program ended but the memory is not
//  signaled as available to the operative system. Problematic in cases in which the computer
//  cannot restart or being turn off as in servers often happens.
//  - Wild pointers, pointers that are used without assigning an address, contain an unpredictable
//  value that is considered by the program as an address. It can cause undefined behaviour or
//  crash.
//  
//  Proper memory allocation is DIFFICULT!
//  Often tools as valgrind and Dr.Memory are used to check if the program written by ourself is
//  referencing memory in a risky way.
