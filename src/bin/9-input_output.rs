// ----------------- Input and Output -----------------
//
// Files are abstractions that represent a block of memory binded to a name.
// Each operative system provides a file system as a tree or files and folders.
// A path is a string of the concatenation of the folder locations and the name of the file.
// Each operative system also defines a system of privileges that establishes permissions to read,
// write and execute files for different users of the operative system.
// Programming languages offer libraries (often in the standard library) to execute actions on the
// file system transparently without knowing the operative system in which the code will be
// executed.
//
// Rust provides an abstraction for files in the standard library (std::fs::File).
// Also C++17 introduced it in std::filesystem library.
// Previously C++ could allow to use this definitions using experimental or boost versions of the
// language.
//
// ------------------ Paths -------------------
//
// Each OS has a definition for which is the folder containing the entire filesystem and
// how folders should be concatenated to be a valid path.
// Rust provides two data structures to represent paths: Path and PathBuf.
// The Path struct represents a valid path to a file or directory that is only readable
// using as internal data structure the str.
// The PathBuf struct represents a path that is modifiable concatenating more names to the path and
// is internally represented by a String.
// Both this types allow to read files and folders metadata as dimensions, creation time, last
// modification time, permissions and more, as well as reading the type of the pointed element by
// the path file, folder, link to something.
//
// The std fs library provides methods to operate on the file system as:
// - read_dir(&Path) -> Result<ReadDir>, which allows to read the content of a folder returning if
// succeded a type readdir which is an iterator composed by DirEntry that can be either file, folder
// or link.
// - create_dir(&Path) -> Result<()>, which allows to create a folder in the path.
// - remove_dir(&Path) -> Result<()>, which removes the specified folder checking if is empty and
// the current user has permissions to delete it.
// - copy(from: &Path,to: &Path) -> Result<i64>, which copies the content of a file from one path
// to another and returns the number of bytes written.
// - rename(from: &Path,to: &Path) -> Result<()>, which renames/moves a file from one path to another.
// - remove_file(&Path) -> Result<()>, which removes a file from the path.
//
// Furthermore, the std fs library contains methods to create an instance of a file handler which
// is required at execution time to cooperate with actual file system in use. The methods are:
//
// - File::open(path) -> Result<File>, which opens a file in read-only mode.
// - File::create(path) -> Result<File>, which creates a file in write-only mode.
//
// Is possible to open files with rights to write or execute using another type in the std::fs
// library called OpenOptions.
//
// The operations available on files are generally comparable to the std::fs methods:
//
// - read_to_string(path: &Path) -> Result<String>, which reads the content of a file and returns it as
// a string.
// - write(path: &Path, contents) -> Result<()>, which writes the content of a string to a file.
//
// ---------------------- IO Prelude ----------------------
//
// The fs library implementation is based largely on the std::io library which contains type, data
// structures and methods not only to interact with the file system but also with internet and
// other peripherals.
//
// The whole set of methods and type available in the fs library is based on the implementation of
// the traits: Read, BufRead, Write and Seek. Their declaration is contained in std::io::prelude.
//
// The errors which will be produced by previously described fs methods are based on the enum std::io::ErrorKind which can
// assume values as Interrupted, NotFound, PermissionDenied, etc.
// 
// Traits as Read, Write and BufRead are implemented by many types of rust libraries such as Stdin,
// File, TcpStream, Stderr, BufReader, BufWriter and many more.
//
// READ
// To implement the std::io::Read trait the read(buf: &mut [u8]) -> Result<usize> method must be
// implemented. Once this is done the type which has the Read trait implemenented will be able to
// use methods as:
//
// - read_to_string(buf: &mut String) -> Result<usize>, which reads the content and returns the
// number of bytes red or an error.
// - read_to_end(buf: &mut Vec<u8>) -> Result<usize>, which reads the contents till the buffer is not empty or an error occurs and returns the number of bytes red or an error.
// - read_exact(buf: &mut [u8]) -> Result<()>, which reads exactly the number of bytes specified in the buf argument or an error occurs and returns an empty tuple if succeded or an error if not.
// - bytes() -> Bytes<Self>, which returns an iterator over the bytes.
// - chain<R: Read>(reader: R) -> Chain<Self, R>, which allows to concatenate the current reader
// to another so that they can be then red in sequence.
// - take(limit: u64) -> Take<Self>, which returns an iterator that will take at most the number of limit bytes specified.
//
// BUFREAD
// Is a trait that can be used to read from sources which are not emitting data always at the same
// rate so that the read operations can be invoked at different times to not block the execution of
// the program waiting for the read completion. So it can be useful if reading internet responses
// packets or any asynchronous input sources such as hard drives, network connections or in general io
// devices. So files which are often on the same system where the program is executed can avoid
// using this trait or they can using it if their size is so big that they will block the execution
// of the program to not ruin "the user experience".
//
// To implement the std::io::BufRead a type should implement the methods:
//
// - fill_buf() -> Result<&[u8]>, which returns the next bytes to be read.
// - consume(n: usize) -> Result<()>, which consumes the first n bytes of the buffer so that on the
// next read the result will not be the same of the last one.
//
// Once they are implemented the type can use methods:
//
// - read_line(&mut self, buf: &mut String) -> Result<usize>, which reads the line from the buffer
// and returns the number of bytes red or an error.
// - lines(self) -> Lines<Self>, which returns an iterator over the lines of the buffer.
// - has_data_left(&mut self) -> bool, which returns true if the buffer is not empty.
// - read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> Result<usize>, which reads the content
// until the number of specified bytes.
// - skip_until(&mut self, byte: u8) -> Result<usize>, which skips the content of the buffer until
// the specified number of bytes.
// - split(byte: u8) -> SplitBytes<Self>, which returns an iterator over the bytes of the buffer
// separated by the specified byte.
//
// WRITE
// To implement the std::io::Write trait is required to define for the type the methods:
// - write(buf: &[u8]) -> Result<usize>, which writes the content of the buffer and
// returns the number of bytes written or an error.
// - flush() -> Result<()>, which flushes the content of the buffer.
//
// When done is possible to use methods:
// - write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> Result<usize> { ... }, which writes the
// conent of the IoSlices to the buffer.
// - is_write_vectored(&self) -> bool { ... }, which specifies if the buffer contains chucks to be
// written.
// - write_all(&mut self, buf: &[u8]) -> Result<()> { ... }, which writes the content of the buffer
// to the destination output.
// - write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> Result<()> { ... }, attempts to
// write each IoSlice to the destination output.
// - write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()> { ... }, attempts to write the
// specified string to the destination output.
// - by_ref(&mut self) -> &mut Self, which returns a reference to the destination output.
//
// SEEK
// Allows to reposition the cursor contained in a buffered reader of writer.
// Is implementable by types implementing the seek method:
//
// - seek(&mut self, pos: SeekFrom) -> Result<u64>, which repositions the cursor in the buffer
//
// Also an enum called SeekFrom has to be imported from std::io.
//
// Using data structures/types that implement Seek allows to use methods as:
//
// - rewind() -> Result<()>, which repositions the cursor to the beginning of the buffer.
// - stream_len() -> Result<u64>, which returns the length of the buffer.
// - stream_position() -> Result<u64>, which returns the current position of the cursor from the
// start of the buffer a.k.a the number of bytes red so far.
// - seek_relative(offset: i64) -> Result<()>, which repositions the cursor by the specified number
// of bytes after the start.
// 
// Most of the BufReader and BufWriter types in the standard library implement Seek.
//
