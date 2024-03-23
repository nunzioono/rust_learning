// ------------- Collections -------------
//
// All the programming languages offer in their respective standard library, a set of data
// structures that can be used to store information efficiently with state of the art coding 
// techniques without reimplement them for each program.
// 
// If these data structures have multiple possible implementations (each one with advantages for a
// specific problem) then they are included in the standard library.
//
// Is tho responsibility of the developer to understand which collection fits best the kind of
// problem he is facing.
//
// ------------- Collections in Rust -------------
//
// Rust implements several data structures in the standard library like:
//
//   Data structure, Description, Access Complexity, Search Complexity, Insertion Complexity,
//   Cancel Complexity
// - Vec, a dynamic list of elements that grows from the end, O(1), O(n), O(n), O(n)
// - VecDeque, a list with efficient insertion and removal at both ends, O(n), O(n), O(1), O(1)
// - LinkedList, a list with efficient insertion and removal at both ends and each node has access
// to his predecessor too, O(n), O(n), O(1), O(1)
// - BinaryHeap, a priority queue, O(1), -, O(log(n)), O(log(n)) 
// - HashMap, a hash table, O(1), O(1), O(1), O(1)
// - BTreeMap, an ordered map, O(log(n)), O(log(n)), O(log(n)), O(log(n))
//- HashSet, an unordered set, -, O(1), O(1), O(1)
// - BTreeSet, an ordered set, -, O(log(n)), O(log(n)), O(log(n))
//
// Each of these data structures provides the following methods:
//
// - new, to allocate in the heap a new collection
// - len, to get the number of elements
// - clear, to remove all the elements
// - is_empty, to check if the collection is empty
// - iter, to iterate on the elements contained in the collection
//
// Also all the data structures implement the following traits:
//
// - IntoIterator
// - FromIterator
//
// Consequently having the methods into_iter() and collect() to cast a collection into and from a 
// iterator respectively.
//
// --------------- Vec ---------------
//
// The Vec data structure is a dynamic list of elements that grows from the end.
// It is represented has a tuple with 3 private fields.
// A pointer to the starting memory address.
// An integer without sign to indicate the length of the memory allocated.
// An integer without sign to indicate how many elements are currently in the vec itself.
// Is the standard way to use collections.
//
// METHODS AND OPERATIONS:
// push(), inserts a new element at the end, if the memory is not enough the buffer will be
// reallocated and the element added then to the end.
// &vec_name[i], get(i) and get_mut(i) allows to get an option containing if succeded a reference to the content of the vec at the
// i-th element.
// pop(), removes and returns the last element.
// insert(index, value), inserts the value at the index position.
// first() and first_mut(), return a reference to the first element.
// last() and last_mut(), return a reference to the last element.
// get(range) and get_mut(range) return an option to a slice in the range specified.
//
// --------------- VecDeque ---------------
//
// The VecDeque models a queue extensible from the start and/or from the end.
// It gets allocated on the heap.
// Differs from the Vec since the methods to get references to the elements at the ends are called
// push_front(value), pop_front(), push_back(value), pop_back().
// It's useful if is needed a big number of operations on the first element of the queue.
// It's a circular buffer and gets allocated on different addresses, to make it allocate only
// contigous memory use make_contiguous().
//
// --------------- LinkedList ---------------
//
// The LinkedList data structure is allows to store elements on the heap and access them in any
// order starting from whatever element.
// It's substancially a sub-structure of Vec and VecDeque and so is almost always not preferrable
// to the two. It can be advantageous if used in concurrent environments.
//
// --------------- HashMap ---------------
//
// The HashMap data structure allows to store elements in a hash table and access them in any order
// by their keys. It is allocated in the heap but not in contigous memory, also can be reallocated
// and the address then can change at runtime when a new pair is inserted.
//
// --------------- BTreeMap ---------------
//
// Is a collection of pairs on the heap that have keys organized in a binary tree.
// The type of the keys must have the trait Ord implemented. As the HashMap is position in memory
// can change at runtime.
//
// --------------- Entry ---------------
//
// Is the Rust enum representation of the single pair within the HashMap and the BTreeMap.
// It can be used as a way to effectively reduce code written since implements methods as
// and_modify() and or_insert() than on a reference would be able to work only evaluing the
// borrowing rules correctly and doing all the needed checks. An entry can be accessed using the
// entry(key) method on one of the previous illustrated maps.
//
// --------------- HashSet ---------------
//
// Is a collection of elements on the heap that has the same properties as the HashMap.
// It is structured as a wrapped of the HashMap with the logical correspondant of a HashMap<V, ()>.
// Is convienient if want to use an hashmap in which there are no duplicates.
//
// --------------- BTreeSet ---------------
//
// Is a wrapper for BTreeMap<V, ()> and has the same properties as the HashSet. Is useful
// if no duplicates are allowed.
//
// --------------- BinaryHeap ---------------
//
// Is a priority queue that uses the elements ordered so that after each insertion the greatest
// element is always at the first position. The first element can be accessed with the method
// peek() or peek_mut().
// The type of the elements inserted must implement the trait Ord.
//
//
