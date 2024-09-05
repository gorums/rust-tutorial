Using Structs to Structure Related Data

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes. In this chapter, we’ll compare and contrast tuples with structs to build on what you already know and demonstrate when structs are a better way to group data.

Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

```
let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

user1.email = String::from("anotheremail@example.com");
```

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.  As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7. The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

The code in Listing 5-7 also creates an instance in user2 that has a different value for email but has the same values for the username, active, and sign_in_count fields from user1. The ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

Note that the struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the “Variables and Data Interacting with Move” section. In this example, we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2. If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.

Using Tuple Structs Without Named Fields to Create Different Types
Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.

Unit-Like Structs Without Any Fields
You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section. Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!

In addition to the Debug trait, Rust has provided a number of traits for us to use with the derive attribute that can add useful behavior to our custom types.

Method Syntax
Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object) and their first parameter is always self, which represents the instance of the struct the method is being called on.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

Associated Functions
All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the String::from function that’s defined on the String type.

Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new, but new isn’t a special name and isn’t built into the language. For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

```
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example.

 This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules.

 Multiple impl Blocks
Each struct is allowed to have multiple impl blocks. 

Summary
Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.