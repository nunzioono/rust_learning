// -------------- Errors ---------------
// 
// All computation can fail prematurely. Making so that an attended value is not returned or that some collateral effect is not executed. This failures can be due to different causes:
//
// - Illegal arguments, failed conversion, etc;
// - Resource exhaustion (memory, disk or network related issues);
//
// Indipendently from the causes of a failure, they can be classified as recoverable or
// unrecoverable.
// In the case of an unrecoverable failure, the program should terminate with an error message.
// In the case of a recoverable failure, the program should continue with the execution after
// recovering the state from the failure, recovering implies a strategy as reattempt the operation,
// require admin privileges or other strategies.
//
// Often happens that recoverable errors are raised in lines of code where there is not enough
// context to determine the strategy to recover so is needed that any programming language allows
// the developer to return the error to the caller of the function.
// This inevitably introduces complexity in the program, adding new branches (if/else, switch/case,
// match...) along the computation can continue but favouring logical errors trying to fix the
// ones that were raised in the first place. Therefore in modern languages the concept of
// exceptions is used to support error handling without adding too much code.
//
// --------------- Syntax in other languages ---------------
// 
// C does not support the developer with any construct to handle errors.
// C++ like many others (Java, C#, Javascript, ...) allows the developer to use exceptions.
// These construct offers the keywords `try`, `catch` and `throw` to define code blocks that
// respectively are executed always (but not completely if an error occurs), when an error is
// matched and when an error is thrown.
//
// --------------- Syntax in Rust ---------------
// 
// Rust utilizes Result and Option generic types to map both the success and failure of an
// operation and the presence or not of a result. Furthermore, it makes available the macro
// panic!(...) (which acts like println!()) to force the program termination with a specific error
// message.
//
// RECOVERABLE ERRORS
// Returning to the old problem of giving a context to the recoverable errors, Rust takes the
// approach of C++ of catch keywords to handle errors proposing a version not based on exceptions
// but using a type system based on the extension or the basic Error type. Since errors are data
// types they can be inspected using match or if let. That are useful with Result and Option since
// they are implemented as enums. These data structures also implement methods to get result values
// or errors if they exist like "is_ok()", "is_err()", "ok()", "err()". Or if a recovery strategy
// has to be implemented based on the error functions like "map()" and "contains()" are useful to
// this scope.
//
// UNRECOVERABLE ERRORS
// If no strategy has to be implemented "unwrap()" is used to extract the result or
// fails exiting the program. The termination of the program should still do some kind of behaviour
// when an error occurs. Even if the error is not recoverable, the program should exit releasing
// all the resource acquired to not determine malfunctions in end-user system (Like memory leaks,
// file corruption, etc).
// So Rust expects the programmer binding this release operations while the data is released using
// the Drop trait. Panicing a program still executes drop operations for the data in the opposite order the scopes were created.
//
// Sometimes is possible to ignore some errors, cause they are already prevented but a specific
// library for example returns still a Result type. This can be done using the "except()" method
// which allows to print a specific error message while not exiting.
//
// REDUCING THE COMPLEXITY OF ERROR HANDLING
// Rust offers a convinient operator to explain the compiler the correspective or the unwrap
// function, which is the ? operator. This can be used in functions that have Result as return
// type and returns on a line the value contained in the Ok branch of the Result or the error
// contained in the Err branch. Note that the same can be done with Option.
//
// THROWING ETHEROGENEOUS ERRORS
// Since a function can fail in multiple istructions it is possible to enumerate the errors that
// can happen within the function and return through a Result type. This requires to implement an
// enum type for the custom error, putting no each branch the specific error type and than provide
// a way to cast the general enum to each one of these branches (error types). The Error trait
// comes in help defining the From trait which allowes with the method "downcast_ref()" to map
// every error to a Box<dyn Error> type.
// Tho our enumeration should implement for now the Error trait, the From trait and obviously
// Display and Debug traits to extract a string to print when the error gets raised.
//
// EXTERNAL CRATES TO REDUCE ETHEROGENEOUS ERRORS COMPLEXITY
// This time to reduce the complexity of this operation external crates can be used.
// 
// FOR LIBRARIES OR DETAILED ERROR HANDLING
// The crate "thiserror" allows with macros to derive traits from the Error trait.
// Preceding the error with the attribute "#[derive(Error, Debug)]" for example it gets an
// automatic implementation for those traits. Also useful the attribute "#[error("Error message
// string...")]" before one of the enum branches specifies the error message to print. Lastly the
// attribute "#[from]" before of a branch of the enum implements automatically the cast from the
// specific error to the more general Box<dyn Error> type.
// 
// FOR APPLICATIONS
// If is not needed to provide a specific implementation of an etherogeneous error type, the crate "anyhow" can be used to return a specific Result type defined within the crate that offers methods like "context(...)" and "with_context(...)" to specify respectively message errors to print in case of errors without a variable or with it.
// For example if an error message contains a value produced at runtime this should be printed
// using the with_context method providing a lambda function that takes as argument the runtime
// value and returns a string while if the message error doesn't contain any runtime value can be
// implemented with context method passing it directly the string to print.
