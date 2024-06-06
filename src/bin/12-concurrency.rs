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
// 
// If needed rust allows the programmer to configure the thread creation through a builder.
// The std::thread::Builder struct allows to specify parameters like the thread name or the thread
// stack size.
//
// As stated above rust helps the programmer forcing his code to behave correctly.
// Traits Send and Sync are part of the cohercing method rust adopts.
// The trait Send is inherited by all the data structure which can be safely shared between
// threads because they cannot be red and/or written by multiple threads at the same time.
// Pointers and reference DO NOT implement the Send trait. So Sync is a trait that gets
// auto-implemented (as Send) by &T when a generic type T implements Send.
// Composed types like structs, enums and arrays of types that implement the Send trait "inherit"
// the Send trait for themselves.
// If a type gets passed by reference (immutable) to other threads it has to implement then the
// Sync trait and if it doesn't the compiler will avoid the code to compile.
// 
// !!!Is important to note that types that allow interior mutability like types which contain Cell
// or RefCell do not implement the Sync trait and so references to them cannot be shared between different threads!!!
// ^
// | This is a perfect example of how Rust coherces the concurrent code written by the programmer to not be
// dangerous.
// 
// In rust many ways to realize synchromization methods between different threads are acheivable.
// In the standard library are contained structures and methods to acheive two basics models to
// make threads concurrent on a shared data structure:
// 1. Synch threads when accessing shared data. (locks-like mechanisms)
// 2. Synch threads when a given critical section is done. (message based mechanism)
//
// If the programmer wants to synch his concurrent code in other ways is also possible to use
// already-made libraries like: actix (synch through an actor model), rayon (work stealing model)
// and crossbeam (share data from a parent thread to his childs).
//
// ------------------------------ ACCESSING CRITICAL SECTIONS THROUGH LOCKS -----------------------
//
// Rust allows this approach giving the programmer Mutex constructs. A Mutex (MUTual EXclusion lock),
// allows the programmer to lock the access of a data contained in the stack to a single thread and
// then release it when the critical operation is done. Mutex implements methods like lock() and
// unlock() to do the operations just described. When a different thread from the one which is
// operating on the mutex data tries to lock the same resource it gets paused (enters a loop) and
// awaits for unlock on the resource from the thread which is using the same resource requested.
// Is tho true that making a concurrent program in which each thread waits indefinetly for a
// resource reduces parallelism making the program non-concurrent in a extreme case.
// So programmer responsiblity is to assess which data should be locked and where this practice is
// necessary.
// The lock() method returns a LockResult<MutexGuard<T>> which can contain either the mutex guard
// to the resource acquired or an indefinite wait (following the same implementation of Result<T,
// Error>,  Result<Guard, PoisonError<Guard>>).
// When the result is_ok(), the content is MutexGuard which is a struct implementing the Deref
// trait, this allows to obtain a mutable reference to the contained data.
// An important thing to note is that to generate a MutexGuard using the Mutex.lock method is that
// the Mutex should be owned from the caller thread. This so allows only a thread at time to
// operate on it.
//
// To use this mechanism allowing to operate in multi-threaded fashion a struct which operates in a
// similar way is given, Arc<T> (Atomic Ref Cell). std::sync::Arc is a struct used as a wrapper that allows to 
// make clones of a data keeping count of the references available across multiple thread so that
// when the last reference goes out of scope the struct deallocates the memory which contains the
// wrapped data.
// So in a multi-threaded envoirment a shared lockable data appears as:
//
// Arc::new(Mutex::new(THE DATA))
//
// MAKING THE COMPILER UNDERSTAND LIFETIMES ACROSS MULTIPLE THREADS
//
// As stated in the title is not possible for the compiler to understand whether a reference has a
// valid lifetime and so points an existing data.
// Rust offers a function std::sync::scope() which takes a lambda function in which is possible to
// spawn threads having the compiler checking lifetimes. This is achieved internally making the
// thread representing the lambda function wait for the completion of each one of the threads
// spawned inside of it.
// The most common usage of this tecnique is sharing a common state across threads, here is an
// example:
//
// "
// use std::thread::{scope, thread};
//
// fn main() {
//     let mut v = vec![1, 2, 3];
//     let mut x = 0;
//     thread::scope(|s| {
//         s.spawn(|| { // è lecito creare un riferimento a v
//             println!("length: {}", v.len());
//         });
//         s.spawn(|| { // anche qui viene catturato &v
//             for n in &v {println!("{n}"); }
//             x += v[0]+v[2]; // x è catturata come &mut 
//         });
//     });
//     // Solo quando entrambi i thread saranno terminati si proseguirà
//     v.push(4); // non ci sono più riferimenti, si può modificare
//     assert_eq!(x, v.len());
// }
// "
// 
// READ/WRITE LOCKS
//
// Another common practice to improve performance in multi-threaded env, is locking resource based
// on the operation to perform. In practice if the most common operation on a data for example is
// reading it and occasionally some thread writes to it is more usefull to implement a RwLock
// instead of a Mutex. What RwLock does is to capture the data passed to it and make a thread lock
// the read or the write on it. Is important to note that if a thread locks the data to write it no
// other operations can occour while if a thread locks the data to read it other threads can also
// lock it to read it. This reduces the "friction" from having parallel processing and making
// threads wait for a resource.
// RwLock::new(DATA) is the way to wrap a data inside this kind of lock.
// RwLock.read() is the method to lock on reading.
// RwLock.write() is the method to lock on writing.
// As it happened for Mutex locking the resource returns a LockResult<SOME KIND OF GUARD>,
// which can either return the guard to the data or making the thread wait for the resource unlock.
// For RwLock two types of guards exist, RwLockReadGuard and RwLockWriteGuard.
//
// ATOMIC TYPES - A MORE PRACTICAL READ/WRITE LOCK WAY
//
// std::sync::atomic module contains primitive types as AtomicU8 for example which implement
// barriers internally so that they shouldn't be wrapped in any kind of Mutex or RwLock.
// Given the compiler still checking the ownership of variables they have to be wrapped in Arc tho
// to be shared between different threads.
// As for primitive types in single-threaded contexts their modification can happen dereferencing
// an immutable reference so they don't need to be passed as &mut.
// An example in which a usize in his atomic version is shared between two threads and while one
// writes 0 in it the other waits it to be modified and then ends:
//
// "
//  use std::sync::Arc;
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::{hint, thread};
// fn main() {
//     let spinlock = Arc::new(AtomicUsize::new(1));
//     let spinlock_clone = Arc::clone(&spinlock);
//     let thread = thread::spawn(move|| {
//         spinlock_clone.store(0, Ordering::Release);
//     });
//     // Attendi 
//     while spinlock.load(Ordering::Acquire) != 0 {
//         hint::spin_loop();
//     }
//     thread.join().unwrap();
// }
// "
//
// CIRCULAR DEPENDENCIES
//
// A problem not immediately obvious from the implicit operation of reference counters
// such as Rc and Arc is that if they contain references to themselves, even if a
// function of a data type contained in them holds a reference to them, the structure will never
// be deallocated as the reference count cannot reach 0. A construct offered by Rust to solve this
// circular dependency problem is std::sync::Weak (std::sync:: for Arc and std::rc:: for Rc).
// The Weak structure allows to keep a count of the actual clones of a datum and not references to it, this
// structure can be derived from an Arc type by invoking the downgrade() method.
// Conversely having a Weak and wanting to get an Arc will invoke a medoto called
// upgrade(). Note that the value type returned by upgrade is Option<Arc<T>> as
// it is not always possible to upgrade.
// 
// CONDITION VARIABLES  
//
// Often a thread has to wait for the completion of multiple operations/multiple conditions that
// has to be verified. Rather than making locks all over resources needed in conditions to be
// assessed, OS'es offer a mechanism called condition variables. The advantage of using such
// structures is that instead of pooling for the check of a condition this mechanism allows the
// thread to automatically start when the condition is true and also while waiting not consume any
// cpu cicles. In this mechanism each thread which is capable to change the current value of the
// condition variable has the responsibility of sending a notification to the others if the
// condition changes, so to make the condition accessible from a thread per time it has to paired
// with a Mutex.
// In C++ the condition variable mechanism is accessed through the library std::condition_variable,
// in Rust std::sync::CondVar makes the same.
// To make a CondVar work, it gets created with the method new(), it imposes the wait to the
// calling thread through wait() and notifies the updated condition variable state through
// notify_one() or notify_all(). Is important to note that notify_all wakes up all the waiting
// threads but since they should be paired with a mutex they wake up all but one at a time.
// Another important detail of the internal implementation is that a condvar internally keeps
// a list of the waiting threads for the given condition, initially the list is empty and on every
// wait call the caller thread get added to the list. A notification consumption removes a thread
// from the internal list. This mechanism has to be understood by the programmer since there is no 
// explicit way to force the correct execution whitin a construct so the implementation across
// different threads is a programmer responsibility.
// An example of usage of a condition variable in Rust:
//
// "
//    let pair = Arc::new((Mutex::new(false), Condvar::new()));
//    let pair2 = Arc::clone(&pair);
//    // Inside of our lock, spawn a new thread, and then wait for it to start.
//    thread::spawn(move|| {
//    let (lock, cvar) = &*pair2;
//    let mut started = lock.lock().unwrap();
//    *started = true;
//    // We notify the condvar that the value has changed.
//    cvar.notify_one();
//    });
//    // Wait for the thread to start up.
//    let (lock, cvar) = &*pair;
//    let mut started = lock.lock().unwrap();
//    while !*started {
//      started = cvar.wait(started).unwrap();
//    }
// "
//
// An example of the importance of the programmer implementation is given by the existence of the
// following problems: false (1) and lost (2) notifications.
// 
// (1) FALSE NOTIFICATIONS
// Given the concurrent nature of threads once a notification is emitted is rare but possible that one
// thread enters the critical section guarded by the condition variable and before it has the
// possiblity to lock the condvar for himself another thread enters the same condition giving to
// the first the behaviour of a false notification. A method to reduce the occurrence of this behaviour is to use the
// wait_while() method instead of wait(). This method double checks the condition entering and
// exiting the "unlocking" phase so that if another thread in that specifing moment surpasses the
// first in the "acquiring lock" logic execution the second check allows the first thread to not
// falsely acquire the unlock from the condition variable.
// The wait_while() method internal implementation therefore corresponds to the following pseudo-code:
//
// "
//  while condition(&mut *guard) {
//      guard = self.wait(guard)?;
//  }
//  Ok(guard)
// "
// 
// (2) LOST NOTIFICATIONS
// Always thinking of the natural implication of concurrency, given two threads in which one emits
// notifications and the other consumes them if the "emitter" is faster than the "consumer" is
// possible to have lost notifications since the consumer can be still executing a previous
// notification while the second notification is sent.
// There are two possible implementation to attempt to reduce the occurrences of this behaviour.
// One implementation is similar to the solution of false notification, the "consumer" thread uses
// wait_while() double checking the condition on the exit but this time the second check should be
// made at the end of the exucution of the critical task, to make so that instead of going to wait
// again the "consumer" repeats again the critical task if a second notification has occurred while
// doing the first.
// The second approach is philosophically pro-active from the "consumer" point of view.
// Instead of waiting to get a notification, the consumer wakes up automatically each time a
// timeout expires without the needing of a notification. This approach is beneficial in context
// where is guaranteed that a notification should occour, but if this is not the case obviously can
// lead to a false notification problem.
// Rust offers methods such as wait_timeout() and wait_while_timeout().
// The wait_while_timeout() method combines the two described approaches to the problem, making the
// "consumer" thread automatically waking up after a timeout expires and making it check if the
// condition has become true, if yes executes the critical task and if not returns to sleep.
//
// --------------------------------- ACCESSING CRITICAL SECTIONS THROUGH MESSAGES ------------------
//
// Rust offers the programmer to handle some sort of communication between threads even not using a
// shared data state but a message mechanism. In this synch method three entities are given by rust
// to ensure the synchronization between the threads happens correctly: Sender, Receiver and
// channel(). Before threads are created the channel() method gets invoked returning a tuple
// containing a Sender and a Receiver instance. Sender can be cloned. Then when the threads get
// created each one can take a Sender and only one of the takes the Receiver. According to the kind
// of data/message the programmer makes the threads send and the logic of the receiver it's
// possible to implement any kind of synchronization mechanism. The Receiver struct has a method
// called recv() which makes the thread wait until a message is received. If the thread that
// acquires the Receiver ends each of the threads that have acquired the Sender will unleash a
// SendError<T> while if the "senders" threads all ends and the "receiver" thread is still alive,
// this "receiver" thread will unleash a RecvError.
// Similarly to the Lock Mechanisms described before is possible that different messages gets sent
// but the receiver is not yet consuming them. The function channel() just described has an
// infinite capacity for those messages, an example of channel behaviour:
//
// "
//  use std::sync::mpsc::sync_channel;
//  use std::thread;
//  let (tx, rx) = channel();
//  for _ in 0..3 {
//  let tx = tx.clone();
//  // cloned tx dropped within thread
//  thread::spawn(move || tx.send("ok").unwrap());
//  }
//  // Drop the last sender to stop `rx` waiting for message.
//  // The program will not complete if we comment this out.
//  // **All** `tx` needs to be dropped for `rx` to have `Err`.
//  drop(tx);
//  // Unbounded receiver waiting for all senders to complete.
//  while let Ok(msg) = rx.recv() {
//  println!("{}", msg);
//  }
// "
//
// If instead the envoirment of the threads requires that messages cannot be queued in the channel
// without the receiver consuming a certain ammount the function sync_channel(n) allows to create
// a channel which has a capacity of n messages, when a "sender" thread tries to send a message
// on this kind of channel and the capacity has been already reached the "sender" thread gets blocked
// waiting to the message slots being freed from the receiver.
// Obviously the recv() method of the receiver frees one place in the channel storing queue.
// If the sync_channel function receives a parameter 0, the channel is called a rendezvous this
// special behaviour makes so that each written message blocks all the senders until the receiver
// hasn't red the sent message. An example of a sync channel:
//
// "
//  use std::sync::mpsc::sync_channel;
//  use std::thread;
//  let (sender, receiver) = sync_channel(1);
//  // this returns immediately
//  sender.send(1).unwrap();
//  thread::spawn(move|| {
//  // this will block until the previous message has been received
//  sender.send(2).unwrap();
//  });
//  assert_eq!(receiver.recv().unwrap(), 1);
//  assert_eq!(receiver.recv().unwrap(), 2);
// "
//
// ------------------- EXTERNAL LIBRARIES FOR CONCURRENT ENVOIRMENTS -----------------------
// Additionally on top of the existing rust standard features several libraries have been created
// by the rust community.
//
// CROSSBEAM
// 
// Crossbeam is a well-documented library that is actively maintained offering several additional
// features like:
//
// - Atomic constructs, implementations for more rust primitives like an atomic version of the standard Cell,
// crossbeam::atomic::AtomicCell allows to make interior mutability possible across multithreaded
// env.
// 
// - Concurrent data structures, like crossbeam::deque::{Injector, Stealer, Worker} for creating
// schedulers for stealing the free time of the cpu maxing the parallel behaviour or like
// crossbeam::queue::{ArrayQueue, SegQueue} that allow to implement message queues with multiple
// producers and multiple consumers in both bounded and unbounded versions.
//
// - MPMC channels, as for the data structures (general state sharing synchronization methods)
// crossbeam offers mechanisms to extend standard messaging synchronization with multiple producers
// and multiple consumers both bounded and unbounded versions (crossbeam::channel::{bounded(...),
// unbounded(...)}). Functions like crossbeam::channel::{after(...), tick(...)} extend instead the
// timed behaviour in the standard wait_timeout() fashion of lock mechanisms but for channels.
//
// More powerful patterns are introduced by this library making rust concurrent developement even
// easier. Some concepts are:
// - Fan-in / Fan-out, which makes possible distributing tasks to different threads and than
// collect the results on a single scope. Example:
//
// "
//  fn worker(id: usize, rx: Receiver<i32>, tx: Sender<String>) {
//      while let Ok(value) = rx.recv() {
//          tx.send(format!("W{} ({})", id, value)).unwrap();
//      }
//  }
//
//  fn main() {
//      let (tx_input, rx_input) = bounded::<i32>(10);
//      let (tx_output, rx_output) = bounded::<String>(10);
//      let mut worker_handles = Vec::new();
//      for i in 0..3 {
//          let rx = rx_input.clone();
//          let tx = tx_output.clone();
//          worker_handles.push( thread::spawn(move || worker(i, rx, tx)) );
//      }
//      for i in 1..=10 { tx_input.send(i).unwrap(); }
//      drop(tx_input);
//      while let Ok(result) = rx_output.recv() {
//          println!("Received result: {}", result);
//      }
//      for handle in worker_handles { handle.join().unwrap(); }
//  }
// "
//
// - Pipeline, which makes the tasks serializable to keep an order of the operations while making
// them at the same time as parallel as possible. Example:
//
// "
//  fn stage_one(rx: Receiver<i32>, tx: Sender<String>) {
//      while let Ok(value) = rx.recv() {
//          tx.send(format!("S1({})", value)).unwrap();
//      }
//  }
//  
//  fn stage_two(rx: Receiver<String>, tx: Sender<String>) {
//      while let Ok(value) = rx.recv() {
//          tx.send(format!("S2( {} )", value)).unwrap();
//      }
//  }
//
//  fn main() {
//      let (tx_input, rx_input) = bounded::<i32>(10);
//      let (tx_stage_one, rx_stage_one) = bounded::<String>(10);
//      let (tx_output, rx_output) = bounded::<String>(10);
//      let stage_one_handle = thread::spawn(move || stage_one(rx_input, tx_stage_one));
//      let stage_two_handle = thread::spawn(move || stage_two(rx_stage_one, tx_output));
//      for i in 1..=10 { tx_input.send(i).unwrap(); }
//      drop(tx_input);
//      while let Ok(result) = rx_output.recv() { println!("Received result: {}", result); }
//      stage_one_handle.join().unwrap();
//      stage_two_handle.join().unwrap();
//  }
// "
//
// - Producer / Consumer, making clear roles between threads. Example:
//
// "
// fn producer(id: usize, tx: Sender<(usize,i32)>) {
//    for i in 1..=5 { tx.send((id,i)).unwrap(); }
// }
//
// fn consumer(id: usize, rx: Receiver<(usize,i32)>) {
//    while let Ok((sender_id, val)) = rx.recv() {
//       println!("Consumer {} received {} from {}", id, val, sender_id);
//    }
// }
//
// fn main() {
//  let (tx, rx) = bounded::<(usize,i32)>(10);
//  let mut handles = Vec::new();
//  for i in 0..3 {
//      let tx = tx.clone();
//      handles.push( thread::spawn(move || producer(i, tx)) );
//  }
//
//  for i in 0..2 {
//      let rx = rx.clone();
//      handles.push(thread::spawn(move || consumer(i, rx)));
//  }
//
//  drop(tx);
//  for handle in handles { handle.join().unwrap(); }
// }
// "
//
// ACTIX
//
// Actix is a library extending base rust features implementing the concept of actors.
// Actors concept was first introduced by ErLang in 1973.
// This approach removes the need of locks and general synchronization making threads work
// in parallel on their own. If an actor receives an external message it proceeds to change it's
// behaviour.
