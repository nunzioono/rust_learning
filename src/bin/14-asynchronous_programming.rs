// -------------------- ASYNCHRONOUS PROGRAMMING --------------------
//
// As seen in concurrency programming parallel tasks can be complex, furthermore cpu
// resources are critical in executing the logic a programmer wants for his application.
// Often is more indicated to spend some time and watch at the problem to model and write code
// to, other than concurrency model is available and well supported in rust another way to realize
// parallel execution of an application.
// When concurrent threads for example wait all for a critical resource locked and bounded by the
// computation of an unique result we saw that the program becomes less parallel. In these
// situations the approach of asynchronous programming is suggested.
// Citation from "Zero to production in Rust, 2022, L.Palmieri": "We should use threads when we need to elaborate in
// parallel while we should execute asynchronously when we need to wait in parallel".
//
// The asynchronous approach instead of the concurrent one suggests to not create a pool of threads
// that run and wait for resources to enable their tasks but to define "state machines alike structures" that execute one task
// and when they finish check if a resource is available and if it is become the desired task. 
// Rust supports this programming style offering to the programmer a wide set of primitives, like Future
// that is a structure used to define the above introduced virtual state machine, async keyword
// that indicates to the compiler that a function is asynchronous and therefore his output should
// be a Future, await keyword which indicates when an asynchronous function should be executed and
// the code should block his execution until the function doesn't return a result.
// 
// FUTURE
// All the rust code related to asynchronous programming rotates around the Future trait.
// This trait has only one method called pool(...) which takes two parameters
// (self: Pin<&mut Self> and context: &mut Context) and returns one enum called Pool
// wrapping a generic result type. When a Future gets created (a function declaration preceeded by
// async implies the following) the function given automatically gets an implementation of the
// Future trait which will be written by the compiler (rustc). Every once in a while (depend on
// what the function does) the pool method will be invoked, using the first argument of type Pin 
// to allocate itself on the heap and so not be deallocated on the exit from its execution and the
// context parameter will be used to check if the function result is ready or pending internally
// using a Rust struct called Waker which will use threads to process the function result.
//
// IMPLEMENTING OUR OWN ASYNCHRONOUS STATE MACHINE
// To have an effective asynchronous processing model we should create an enum containing the
// possible states across the tasks execution and then implement on the enum the future trait.
// Rust supports the state machine flow of the asynchronous operations but does not specifies how
// the threads and in general the execution should proceed. More than one external library is
// available to allow the programmer to implement his logic according to the scope of his
// application. The following libraries match the missing standard features:
//
// - Tokio, supports asynchronous operations over file-systems, network-connections, databases and
// more..
// - smol, supports asynchronous lightweight operations and is suitable for low resource embedded
// systems.
// - async-std, offers counterparts of the standard library structures with an async fashion.
//
// TOKIO
// This library initialiazes by default an n number of threads and keeps a queue of tasks to run
// for each thread.
// When a Processor (thread representation within tokio) ends the tasks in its queue it tries to fetch a task from the
// others queue. 
// Tokio library is based on the concept of minimizing the synchronization mechanisms using mostly
// atomic variables to regulate interactions between threads.
//
// To initialize a project using tokio we should specify which tokio version to use within the toml
// file, then we can start using it. It's required to annotate the entry point of the asynchronous
// program with an annotation specifying in it how many threads should be used.
// Then each function corresponding to a task should be passed as parameter to a function
// invokation of tokio::spawn(...) and using the return type (future) it can be awaited if needed.
//
// Another explicit way to declare and wait an async task is given within tokio using the macro
// join!(...) passing to it one or more functions implenting the Future trait (just preceed them
// with await). Is either possible to use the macro try_join!(...) which is used to await tasks the
// return Result types, so they can end with an error.
// A join!(...) example:
//
// "
//  async fn do_stuff_async() { … }
//  async fn more_async_work() { … }
//
//  #[tokio::main]
//  async fn main() {
//      let (first, second) = tokio::join!(
//      do_stuff_async(),
//      more_async_work()
//      ); 
//      // do something with the values
//  }
// "
//
// Is either possible running multiple tasks in the join!(...) fashion to just wait for the first
// to complete using another macro: select!(...). This macro takes a match like construct
// specifying for each task passed a callback operation and when one task ends it matches the
// callback and ends the others in execution. Example:
// 
// "
//  async fn do_stuff_async() { … }
//  async fn more_async_work() { … }
//  #[tokio::main]
//  async fn main() {
//      tokio::select! {
//          _ = do_stuff_async() => {
//              println!("do_stuff_async() completed first")
//          }
//          _ = more_async_work() => {
//              println!("more_async_work() completed first")
//          }
//      };
//  }
// "
//
// Is possible to wait in the current thread using the function tokio::time::sleep(d:
// Duration).await or execute a task and wait at max the specified timeout with the function
// tokio::time::timeout(d: Duration, f: F).await where F is the Future (async function implementing the
// task logic), is also useful to know that the return value of the timeout function is always a Result<T>
// containing the return type T of the Future or an Error of type Elapsed if the timeout elapsed.
// If an heavy task which takes a huge ammount of time is needed the function
// tokio::task::spwan_blocking(...) can be used to ensure that the thread execution is stopped
// until the task is not completed (overusing this can lead to very low performance).
// Tokio offers structures for both sharing state and communicate between threads with messages.
// An example of threads in tokio sharing state:
//
// "
//  use tokio::sync::Mutex;
//  use std::sync::Arc;
//  #[tokio::main]
//  async fn main() {
//      let data = Arc::new(Mutex::new(0));
//      let mut v = vec![];
//      for _ in 0..4 {
//          let data = Arc::clone(&data);
//          v.push(tokio::spawn(async move {
//              let mut lock = data.lock().await;
//              *lock += 1;
//          }));
//      }
//      for h in v { let _ = join!(h); }
//      assert_eq!(*(data.lock().await), 4);
//  }
// "
//
// An example using messages:
//
// "
//  async fn some_computation() -> String { "Some result".to_string() }
//  #[tokio::main]
//  async fn main() {
//      let (tx, rx) = oneshot::channel();
//      tokio::spawn(async move {
//          let res = some_computation().await;
//          tx.send(res).unwrap();
//      });
//      //Do other work while the computation is happening in the background
//      // Wait for the computation result
//      let res = rx.await.unwrap();
//  }
// "
//
// An example using mpsc message channels:
//
// "
//  async fn some_computation(i: u32) -> String { format!("Value {}",i) }
//  #[tokio::main]
//  async fn main() {
//      let (tx, mut rx) = mpsc::channel(100);
//      tokio::spawn(async move {
//          for i in 0..10 {
//              let res = some_computation(i).await;
//              tx.send(res).await.unwrap();
//          }
//      });
//      while let Some(res) = rx.await.unwrap() { println!("{}", res); }
//  }
// "
// 
// An example of broadcast/mpmc channels:
//
// "
//  #[tokio::main]
//  async fn main() {
//      let (tx, mut rx1) = broadcast::channel(16);
//      let mut rx2 = tx.subscribe();
//      tokio::spawn(async move {
//          assert_eq!(rx1.recv().await.unwrap(), 10);
//          assert_eq!(rx1.recv().await.unwrap(), 20);
//      });
//      tokio::spawn(async move {
//          assert_eq!(rx2.recv().await.unwrap(), 10);
//          assert_eq!(rx2.recv().await.unwrap(), 20);
//      });
//      tx.send(10).unwrap();
//      tx.send(20).unwrap();
//  }
// "
//
// A reactive implementation approach/observe/watch channel example:
//
// "
//  #[tokio::main]
//  async fn main() {
//      let (tx, mut rx) = watch::channel("value 0");
//      for i in 0..2 {
//          let mut rx = rx.clone();
//          tokio::spawn(async move {
//              while rx.changed().await.is_ok() {
//                  println!("received: {:?}", *rx.borrow() ); 
//              }
//          });
//      }
//      let d = Duration::from_secs(1);
//      tx.send("value 1").unwrap(); tokio::time::sleep(d).await;
//      tx.send("value 2").unwrap(); tokio::time::sleep(d).await;
//      }
// "
//
//
