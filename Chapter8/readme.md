 * Common Collections

Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.

 - A vector allows you to store a variable number of values next to each other.
 - A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
 - A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

  * Storing Lists of Values with Vectors

The first collection type we’ll look at is Vec<T>, also known as a vector. Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

```
 let v: Vec<i32> = Vec::new();
```
More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation.

```
    let v = vec![1, 2, 3];
```

Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary.

Updating a Vector
To create a vector and then add elements to it, we can use the push method

```
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

As with any variable, if we want to be able to change its value, we need to make it mutable using the mut keyword

Reading Elements of Vectors
There are two ways to reference a value stored in a vector: via indexing or by using the get method. In the following examples, we’ve annotated the types of the values that are returned from these functions for extra clarity.

```
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

Note a few details here. We use the index value of 2 to get the third element because vectors are indexed by number, starting at zero. Using & and [] gives us a reference to the element at the index value. When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.

let’s see what happens when we have a vector of five elements and then we try to access an element at index 100 with each technique

```
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
```

When we run this code, the first [] method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.

When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the vector remain valid. Recall the rule that states you can’t have mutable and immutable references in the same scope.

```
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
```

Compiling this code will result in this error:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {first}");
  |                                     ------- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` (bin "collections") due to 1 previous error
```

The code in Listing 8-6 might look like it should work: why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

* Iterating Over the Values in a Vector
To access each element in a vector in turn, we would iterate through all of the elements rather than use indices to access one at a time.

```
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.

Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker’s rules.

 * Using an Enum to Store Multiple Types
Vectors can only store values that are of the same type. This can be inconvenient; there are definitely use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!

```
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.

* Storing UTF-8 Encoded Text with Strings

We talked about strings in Chapter 4, but we’ll look at them in more depth now. New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8. These factors combine in a way that can seem difficult when you’re coming from other programming languages.

What Is a String?
We’ll first define what we mean by the term string. Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.
String literals, for example, are stored in the program’s binary and are therefore string slices.

The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types. Although this section is largely about String, both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

Creating a New String
Many of the same operations available with Vec<T> are available with String as well because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities. An example of a function that works the same way with Vec<T> and String is the new function to create an instance

```
    let mut s = String::new();
```

Updating a String
A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it. In addition, you can conveniently use the + operator or the format! macro to concatenate String values.

Appending to a String with push_str and push
We can grow a String by using the push_str method to append a string slice, as shown in Listing 8-15.

```
    let mut s = String::from("foo");
    s.push_str("bar");
```

Concatenation with the + Operator or the format! Macro
Often, you’ll want to combine two existing strings. One way to do so is to use the + operator, as shown in Listing 8-18.

```
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

The string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator. The + operator uses the add method, whose signature looks something like this:

```
    fn add(self, s: &str) -> String {
```

Second, we can see in the signature that add takes ownership of self because self does not have an &. This means s1 in Listing 8-18 will be moved into the add call and will no longer be valid after that. So, although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies, but it isn’t; the implementation is more efficient than copying.

If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:

```
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```

At this point, s will be tic-tac-toe. With all of the + and " characters, it’s difficult to see what’s going on. For combining strings in more complicated ways, we can instead use the format! macro:

```
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
```

This code also sets s to tic-tac-toe. The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents. The version of the code using format! is much easier to read, and the code generated by the format! macro uses references so that this call doesn’t take ownership of any of its parameters.

Indexing into Strings
In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error. 

```
    let s1 = String::from("hello");
    let h = s1[0];
```

This code will result in the following error:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:16
  |
3 |     let h = s1[0];
  |                ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`, which is required by `String: Index<_>`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
          for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
  = help: the trait `SliceIndex<[_]>` is implemented for `usize`
  = help: for that trait implementation, expected `[_]`, found `str`
  = note: required for `String` to implement `Index<{integer}>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `collections` (bin "collections") due to 1 previous error
```

The error and the note tell the story: Rust strings don’t support indexing. But why not? To answer that question, we need to discuss how Rust stores strings in memory.

Internal Representation
A String is a wrapper over a Vec<u8>. Let’s look at some of our properly encoded UTF-8 example strings from Listing 8-14. First, this one:

```
    let hello = String::from("Hola");
```

In this case, len will be 4, which means the vector storing the string "Hola" is 4 bytes long. Each of these letters takes one byte when encoded in UTF-8.

The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

Bytes and Scalar Values and Grapheme Clusters! Oh My!
Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]

That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:

['न', 'म', 'स', '्', 'त', 'े']

There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:

["न", "म", "स्", "ते"]

Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.

Slicing Strings
Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.

Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

```
let hello = "Здравствуйте";

let s = &hello[0..4];
```
Here, s will be a &str that contains the first four bytes of the string. Earlier, we mentioned that each of these characters was two bytes, which means s will be Зд.

If we were to try to slice only part of a character’s bytes with something like &hello[0..1], Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`
thread 'main' panicked at src/main.rs:4:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

You should use caution when creating string slices with ranges, because doing so can crash your program.

Methods for Iterating Over Strings
The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes. For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:

```
for c in "Зд".chars() {
    println!("{c}");
}
```
З
д


Alternatively, the bytes method returns each raw byte, which might be appropriate for your domain:

```
for b in "Зд".bytes() {
    println!("{b}");
}
```

This code will print the four bytes that make up this string:

208
151
208
180

Getting grapheme clusters from strings, as with the Devanagari script, is complex, so this functionality is not provided by the standard library. Crates are available on crates.io if this is the functionality you need.

Strings Are Not So Simple
To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data up front. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.