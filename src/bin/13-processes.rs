// --------------------- PROCESSES ------------------------
//
// A process is the unit base for an application within an OS.
// Each process is characterized from a PID, Process ID.
// It defines which memory addresses contain it's variables and it's code.
// Binds the execution of the code to modify only the variables within the address range.
// This concept is called isolation.
// Even if isolated 2 processes can communicate through the file system or other mechanisms.
// The set of possible ways in which 2 processes can "talk" is called IPC (Inter Process
// Comunication).
// Often applications desire to have multiple indipendent address-spaces to make their logic only
// occasionally talk, this is the main reason for an application for being composed by multiple
// processes. Another reason can be given by security to hide to a particularly vulnerable code
// exposed to the usage of an user for example from an internal logic which has to collaborate with
// the user. Each process is associated with at least one thread.
// Windows, Linux and other OSes implement their own logic to handle all the operations on a
// process.
// Rust offers to the programmer a convenient api to write code that manipulate the processes being
// at the same time agnostic of which OS is being executed on.
// The struct Command in rust allows to create a new process. Example in which rust api is used to
// detect the OS and run a terminal:
//
// "
//  use std::process::Command;
//  fn main() {
//      let output = if cfg!(target_os = "windows") {
//          Command::new("cmd")
//          .args(["/C", "echo hello"])
//          .output()
//          .expect("failed to execute process")
//      } else {
//          Command::new("sh")
//          .arg("-c")
//          .arg("echo hello")
//          .output()
//          .expect("failed to execute process")
//      };
//      println!("{:?}", output)
//      //Output { status: ExitStatus(unix_wait_status(0)), stdout: "hello\n", stderr: "" }
//  }
// "
//
// Is also possible to add envoirment variables to the Command struct through the methods env(key, val) and envs(vars).
// Using env_remove(key) and env_clear() is possible to remove env variables from the struct.
// This envoirment variables are conveniently set before the process execution.
// With the get_envs() method is possible to check which env variables are binded to the Command
// struct.
// Is also possible to manipulate the input, output and error streams of data going in/out from a
// process using appropriate methods such as:
// - stdout(...)
// - stdin(...)
// - stderr(...)
// The arguments of the above methods are the kind of way in which the "channels" above will be
// set.
// The options are contained in the struct Stdio:
// - inherit(), will inherit the correspondant "channel" from the process parent.
// - piped(), will connect only the input to the parent and the output to the input of the given
// process to create.
// - null(), the specified "channel" will not produce/receive data.
// An example in which the output of a "echo \"Hello, world!\" " on a terminal is redirected to the
// parent and then used in rust within the output variable:
//
// "
//  fn main() {
//      let output = Command::new("echo")
//      .arg("Hello, world!")
//      .stdout(Stdio::piped())
//      .output()
//      .expect("Failed to execute command");
//  }
// "
//
// Other operations available for a process from the rust api are:
//
// - Change the directory in which the process will start with current_dir(dir).
// - Start a process and check if it returns successfully with status().
// - Start a process without checking when it ends with spawn(), which returns a Result<Child>.
//
// If the creation of a process is successful then the struct Child can be used to communicate with
// the process child.
// Through the methods of Child is possible to read the PID assigned from the OS to the new process
// with the method id(), wait() is used to wait for the end of the process, wait_with_output() is
// used to wait for the end and capture all the output the process would have produced.
// With the method kill() is possible to end the process instantly.
// Is possible either to work on the process in the current execution of the rust program using
// methods like std::process::exit(code), which terminates the process signaling to the OS the exit
// code, std::process::abort which terminates and signals to the OS an arbitrary error code.
// 
// !!! Is important to note that panic!() in the current process does not end the process but frees
// the memory !!!
// 
// The crate sysinfo allows the programmer to retrieve info on the processes in execution on the
// current OS:
//
// "
//  fn main() {
//      let mut system = sysinfo::System::new();
//      system.refresh_all();
//      let name = system.process(Pid::from(1)).unwrap().name();
//      println!("Process with id 1 is {}", name)
//  }
// "
// 
// ORPHAN
// A child process whose parent process terminates before the child terminate is called orphan
// process. In Linux the child takes the PID of the terminated parent.
//
// ZOMBIE
// A child process ending before its parent process invokes wait() is called a zombie process.
// A zombie process is freed of its memory but is signaled to the OS as its PID is reserved, the
// exit code is not provided and the counters of the resources used are not signaled to the OS. 
//
// -------------------------------- INTER-PROCESS COMMUNICATION (IPC) -----------------------------
//
// Normally OS work to make processes isolated, making them not sharing data. They allow tho some
// mechanisms to send data from processes in execution. This set of mechanisms are called IPC.
// Since pointers/references are logical addresses to share data between processes which only know
// their own address space the only option suitable is to write and read to the filesystem. The
// format in which is preferrable to write and read is bytes so that the content can be encrypted
// if the data is sensible and the shared content has the minimum lenght possible.
// 
// SERDE
//
// A super useful crate to map structs and data in Rust into the correspondant format to expose in
// the file system is serde. The serde library/crate allows to derive traits Serialize and
// Deserialize. Implementing these traits allows the programmer to call methods such as:
// - serde_json::to_string(&data_to_serialize);
// - serde_json::from_string(&data_to_deserialize);
//
// INTERPROCESS
//
// Interprocess is a crate that maps the communication methods offered by each single supported OS
// into a unique api in rust, allowing so the programmer to write code that works without knowing
// which OS is being runt on.
//
// ZBUS
//
// Zbus is a crate to map the specific interprocess communication protocol of Linux.
//
