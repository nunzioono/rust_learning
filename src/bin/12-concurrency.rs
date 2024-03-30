// ---------------- Concurrency ------------------
//
// THREADS
// A concurrent program has two or more threads in execution at the same time.
// They have to persue a common objective. This threads can be executed in parallel if the CPU has
// multiple cores and/or the switch in execution under control of a scheduler.
// When a program is created it defaults to a single process having a single thread.
// When the execution has to be parallelized, the main thread requests to the scheduler the
// creation of a another thread. Each thread represents an indipendent computation, it runs the
// instructions on a separate stack and runs till is concluded returning a result or an error.
// The OS and/or libraries allocate the memory and other fisical resources while the scheduler
// decides which core has to be used at runtime to perform each instruction on a given thread.
// All the threads are created with a unique identifier and for each one in use the execution state
// is saved.
// The thread handling can be totally demanded to the OS, in this case they are called native
// threads or demanded to a libray with the support of the OS, in this case they are called green
// threads.
// Both C++ and Rust offer in the std library support for native threads. Both offer external libraries
// for green threads.
// For native threads each OS handles the functionality in a personal flavour tho they still have
// common features:
//
// - When a thread is created it returns an opaque handle to the thread.
// - A thread can be identified with a TID (Thread ID).
// - Each thread can be "waited" till it returns a result or an error.
// 
// The deletion of a running native thread is not implemented by the OS but can happen from the
// code it runs.
//
// WHY THREADS?
// Pros:
// 1. They can be used to avoid stalls while for example waiting for an I/O operation.
// 2. They avoid a much higher overhead given from multi-processing. Infact they can share data
// using pointers since they operate on the same address space while processes should
// communicate using signals or IPC tecniques that a data is ready and then when acknowleged it
// make a copy on the destination process.
// 3. Possibility to use real elaboration power of the whole machine using each core needed.
// Cons:
// 1. The program becomes much harder to write and debug.
// 2. Memory is not anymore considerable as a static deposit, a value we expect to stay the same
// may be changed from another thread.
// 3. A mechanism of synchronization for memory access is needed.
// 4. The order in which instructions are runt is not deterministic since each core has a cache.
// 
// MEMORY MODEL
// When a thread fetches some data from memory it can find a value that:
// - Was written at the start of the program, like a global variable
// - Was previously written by himself
// - Was written by another thread
// The third case is particullarly hard to debug since the cache of the various cores can reorder
// the instructions and then the real global order of execution of the whole program is not
// deterministic.
// The developer is obliged to implement in these cases an explicit synchronization model to make
// threads operations on the memory have the established order of execution.
// 
// OPEN PROBLEMS
// - ATOMICITY: Which operations have to be atomic (not interrupted by another thread), note ideally if all
// wait for all other threads the execution is not anymore in parallel but if there are not
// atomical instructions the program behaviour is umpredictable since it depends on the scheduler
// decision to execute before or after an istruction.
// - VISIBILITY: Under which conditions a thread should know another thread did something?
// - ORDERING: Under which conditions the instructions of a thread can appear in another order to
// another thread?
//
// PROCESSOR ANSWERS
// The x86 family of processors provides "fence" instructions that cannot be stopped.
// The ARM family of processors provides "memory barrier" instructions that are locked till a
// certain result is not provided by another thread.
// These implementations condition programmers to include them in their code to make a predictable
// output from a concurrent execution. Both C++ and Rust are binded to libraries and types that
// follow the processors conventions.
//
// ERRORS
// Making a bad use of a synchronization mechanism results in unpredictable behaviours or
// passive/active blocks due to waiting for other threads result/or waiting for a lock to be released.
// These errors do not necessarily show up quickly but can appear after a certain amount of
// executions of the same program or changing the platform on which the program is running.
//
// THREADS AND MEMORY
// OS keeps a representation of the threads within each process. Each one is marked with a TID, has
// a execution state and a set of parameters to resume/pause the execution of a thread.
// All the threads share global variables, constants, heap and the memory in which is placed the
// code to execute.
//
// EXECUTION AND NON-DETERMINISM
// The code inside a single thread is executed in order so as in a non-threaded environment
// understand what happens "before" and "after". Is tho not possible make assumption at a given
// instruction of a thread A make assumption on the state of progress of a thread B.
// Substantially a thread executes his code in order but is not possible to establish a relative
// speed respect of another thread. 
// Another concept to keep in mind while writing a concurrent program is relative to
// synchronization methods.
// 1. A synchronization method is based on the reach of a "particular" state of a thread to unlock
//    the execution of the others.
// 2. A synchronization method can be based on the lock of the other threads while executing a
//    "particular" instruction on a shared resource.
// 3. A third case regards both the cases and instructions which influence the computation of a
//    "particular" value.
// 
// SYNCHRONIZATION
//
// If two threads try to read/write the same variable a critical race happens.
// In base of conditions that the programmer cannot control like, core's cache state or scheduler
// decisions the program assumes a different behaviour on a thread accessing the same variable
// before the other.
// The access to a shared behaviour should be preceeded by a fence/barrier instruction. The
// variable should be modified only if is not being currently modified by any other thread.
// At the same time we want to avoid a continuous pooling to catch when the variable is not
// modified because it would consume CPU cycles and battery.
// In safe rust is not possible to concurrently access a variable and modified since the ownership
// rules wouldn't allow to compile such a program (more than one thread cannot own at the same time
// the variable).
// Both Rust and C++11 offer libraries that standardize the various OSes synchronization
// mechanisms.
//
// Examples:
//
// WINDOWS STRUCTURES:
// - CriticalSection
// - SRWLock
// - ConditionVariable
//
// WINDOWS KERNEL STRUCTURES:
// - Mutex
// - Event
// - Semaphore
// - Pipe
// - Mailslot
// ...
//
// LINUX STRUCTURES:
// - pthread_mutex
// - pthread_cond
// 
// LINUX KERNEL STRUCTURES:
// - Semaphore
// - Pipe
// - Signal
// - Futex
//
// CORRECTNESS
// Is required to never make so that a thread writes a variable while another is reading/writing on the same one.
// In particular, transitional states do not have to be visible to the threads so that they cannot
// perform read/write operations on a transitional state of the object.
// All the shared objects should enforce this rule keeping an invariant state in between of the
// operation performed.
// Mutations should happen only when the variable state before and after is valid, and the mutation
// is not on a mutating state variable.
// The state of the shared variable should be modified through methods that check that the variable
// is not being modified.
// Almost all languages delegate to the programmer to understand when and where synchronization
// should happen.
// In Rust limitations of the borrow checker on the exclusivity of writing a variable united to the
// traits that model the behaviour of a type when it gets passed between threads (Sync, Send)
// guarantee the correctness of the accesses.
// This limitations to the programmer bringed the concept of "fearless concurrency" to Rust.
//
// SHARED ACCESS
// PROBLEMS
// - Atomicity: which operations have indivisible effects? Write and read operations over the same
// variable have no known order of execution a priori.
// - Visibility: a write operation can be observed by a read operation of another thread? No the
// value in between of the write operation can be different from the initial value and even from
// the expected final value.
// - Orderinig: under which conditions, sequence of operations of a thread are seen in the same
// order from other threads? If the operations of the thread produce the same output in whatever
// order they are executed.
// SOLUTIONS
// - Atomic: a variable that has not intermidiate state while being modified. Limited on integers,
// booleans and pointers. These are offered in the std libraries both in C++ and Rust. Rust does it
// on more abstracted variables providing types that ensure through barriers atomicity.
// - Mutex: offers a synchronization method to grant atomicity to complex types, extend mutual
// exclusion on the threads operations on the same variable. If another thread tries to access a
// shared variable that is being modified it gets forced to wait the end of the modification.
// - Condition variable: in some cases, should wait without consuming CPU cycles - that verifies a
// condition more complex than the mutex relaxation from a thread.
// A condition variable allows to implement this wait, with the condition that the thread which
// blocks the others notifies the others when the operation is over.
// A condition variable can also be used in couple with a mutex.
//
// THREAD USAGE
//
// The API of the OSs allow to create and close threads, synch them or assign them private
// locations in memory. The details of the API on each OS differ enough to make hard to port an
// application written on one API to another platform.
// In 2011 C++ introduced a standard API to create and handle threads so that the API in C++ is
// uniform and makes the application portable on different OSs.
// In Rust is done the same with a major attention to the handle of failures.
//
// THREADS IN C++
//
// The C++ library std::thread provides an object called thread that effectively creates a thread
// when his constructor gets invoked. This object contains an opaque handle to the native thread
// created by the OS. The programmer can use the object methods like join() to wait that the
// thread has terminated or detach the execution from the OS calling the detach() method.
// Once the object gets destroyed the destructor checks that the connected thread has been
// terminated and if not calls the method terminate() to force the termination.
// If the thread terminates for an error it gets closed calling terminate().
//
// THREADS IN RUST
//
// In Rust the library is std::thread, it contains the method spawn() which takes as argument a
// lambda function to specify the code it has to execute and returns an handle with type
// JoinHandle<T> where T is the type the thread will return.
// To wait the thread termination the library provides a method on the struct JoinHandle called
// join(). The join() method returns a Result analog the one often used in non-parallel Rust but
// defined in std::thread::Result since it contains a result or an error only provoked by the
// thread itself. In this way once a thread is created it is totally indipendent from the rest of
// the threads it can survive even if the others terminate. In other words there is no relation
// parend-child between the thread that spawns another one and the thread created. If the thread
// created exits the scope in which was defined it acquires the state detatched and the creator has
// no more ways to communicate with it/retrieve a result from it.
