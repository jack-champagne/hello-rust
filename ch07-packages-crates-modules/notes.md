As the nature of the Book changes, so will I need to adapt the way that I take notes about the code that I am writing. This chapter focuses very heavily on higher level concepts such as project organization and code structure. As such, a lot of the notes and observations could possibly end up in this file.

The module system
* Packages: Cargo features that let you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths.
* Paths: A way of naming an item, such as a struct, function, or module.

### Packages and crates

The first thing that I am working on is going to be Packages and Crates.
 - Crate: A binary or a library.
 - The crate root is the source file that the compiler start from (Also root module of the crate)
 - A package is one or more crates that provide a *set* of functionality (will contain a cargo.toml file that describes how to build its crates)
 - A package must contain zero or one library crates, and **no** more. It can contain as many binary crates as you like though! Will contain at least one crate (either a library or binary)

Now we create a new package (my-project) using this command
```bash
cargo new my-project
```

This will create a Cargo.toml file that constitutes our package. The reason why src/main.rs is never specified as the crate root is because of a Rust convention. src/main.rs will be the crate root to a binary crate with the same name as the package. Under this logic, the crate root to the binary crate named my-project in this directory is my-project/src/main.rs Where my-project package contains the binary crate of my-package.

The same thing basically applies to library crates. If a package directory contains a src/lib.rs then the package contains the library crate with *the same name as the package* and will have src/lib.rs as the crate root. Cargo will end up passing the crate root to rustc to build libraries and binaries.

If our directory were to contain a src/main.rs and a src/lib.rs, then we would have two crates (a binary crate and a library crate) both with the same name as the package. - A package can have muliple binary crates by putting files into the src/bin directory. (Each file will be a separate binary crate)

- A crate groups related functionality together in a scope so that this functionality will be easy to share between multiple projects. The rand crate used in chapter 2 for example provides functionality to generate random numbers. We can use it by bringing the rand crate into our projects *scope*. The functionality provided by a crate can be accessed through the crate's **name**

Rust is smart about scope and namespacing. The language is able to understand the same attribute being defined in two different crates as two *different* attributes for example, the compiler understand the difference between: struct Rng and rand::Rng, where struct Rng is defined in our file.

### Modules

Key concepts:
- Modules: Used to control scope and privacy
- Paths (naming items)
- use keyword
- pub keyword
- as keyword
- external packages
- glob operator

Modules organize codes within crates for easy reuse and readability. They also can control privacy (outside code: public) vs (internal detail not for outside use: private)

To start, let's examine the case of modeling a restaurant by creating a nice library.
```bash
cargo new --lib restaurant
```

front_of_house module is composed of two other modules (hosting and serving). All modules are defined by starting with the mod keyword followed by the name of the module to be specified. Then in curly brackets following that, the body of the module is defined. Modules can contain other modules, structs, enums, constants, traits, and function signatures.

Grouping related definitions together makes code more readable, organized, and easier to use. Navigation of code relies on groups instead of relying upon definitions. The module tree is structured with a module named *crate* as the root, and then the crate root will compose the child of that module.

Here is an example for our restaurant.
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

Modules nest inside of eachother and form a true tree structure. front_of_house is a child of the implicit root module named root.

#### Paths

To discern what item we are talking about when we code with Rust, we must give the items path through the module tree. Similarly to how one would need to give a path to specify a location in a filesystem. Every path follows two forms:
* Absolute: Starts from crate root by using crate name or literal crate
* Relative: Starts from current module and uses self, super, or identifier in the current module.

Both paths are delimited using ::

How to call the add_to_waitlist function is the same as asking, what is the path of the add_to_waitlist function.
Now let's try calling the add_to_waitlist function.

You can think of the crate:: at the beginning of an absolute path like the / (root) directory. The decision to use absolute or relative paths is a decision that is made on a per project basis. Basically, if we were to move the code into a module called customer_expirience, then we would need to update the absolute path but not the relative path. If were were to move the eat_at_restaurant function into a new module called dining, the absolute path would still work but the relative one would not.

**Absolute paths are preferential** because it is more likely that definition and usage are likely independent of eachother.

Here when we try to compile the library, we get an error because we try to access a non-public module 'hosting' and call one of its inner fields. This is the first example we have of a privacy boundary. All items are private be default and all item 'beyond' the privacy boundary are considered internal implementation details that outside need not rely upon. If I want to make a struct or function private, all I need to do is put it into a module.

Parent module cannot see their children's private internals but a child module can see all details of its ancestors. This is because it is useful for the child modules to see the context in which they are defined.

Now we should add the pub keyword to the module 'hosting' in order to use the path from earlier. Notice that we must also expose the private internal details of the hosting module by attaching a pub keyword to a function that we try calling from outside code 'add_to_waitlist()'. More precisely, With the pub keyword on the hosting module, if we can access front_of_house we can also access hosting. We cannot access the contents of hosting however, these contents were never made public. *The pub keyword on a module only lets code in its ancestor modules refer to it*.

Privacy rules apply to: structs, enums, functions, methods, and modules.

Note that the module front_of_house is not public. The reason why our method eat_at_restarant() is able to access the module is because they are siblings. They are defined in the same module which means we can refer to module 'front_of_house' from our function eat_at_restaurant(). 

More notes on absolute paths and such.
The super keyword is similar to the idea of the .. directory for paths. In the written example, we go one module up from back_of_house into the crate module and then call a function named serve_order() from there, which exists! Success!

There is a bit of extra information that is useful when it comes to structs. For a struct, you can make each field public on a case by case basis. 

A good note to be added here. Since the struct for Breakfast has a private field and cannot be directly accessed, we can see how useful a constructor is here! We need it to construct a valid Breakfast object.

Unlike a struct, if an enum is public, so are all of its variants. See Appetizer enum.

### Use keyword

The paths that we have used so far can be shortened! We can bring a path into scope once and then call those items as if they're local items with the keyword use. Check out the second project/package named restarant2. This is very similar to creating a symbolic link in the filesystem, shorthening the path between two items by supplying extra information/bringing something into the scope. It is almost like the hosting module was defined in the crate root. Same privacy rules apply. And!!! This also works with relative paths.

We could have also wrote out a full path to the inner function of hosting like so:
```Rust
use crate::front_of_house::hosting::add_to_waitlist;

pub fn foo() {
    add_to_waitlist();
    add_to_waitlist();
}
```

Instead of hosting::add_to_waitlist();, it looks like this. The previous method in the file makes it clear that the code isn't locally defined. When scoping other items such as structs, enums, and other items, it makes sense to specify full path like so:
```Rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();   // Instead of collections::HashMap::new();
    map.insert(1, 2);
}
```

There is an exception, whenever trying to bring in two items that have the same name, we would not use the full path so that we are able to specify which item we are referring to.
```Rust
use std::fmt; // Both contain item named Result
use std::io;

fn foo() -> fmt::Result {}
fn bar() -> io::Result<()> {}
```

We could also avert this problem by just renaming when we import two items of the same name.
```Rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

Both are idiomatic! The as keyword just renames the item. Next is the idea of Re-exporting. This is where an item that we bring into the current scope we want to be public for other code.
```Rust
pub use crate::front_of_house::hosting;

// This will allow external code to refer to public items inside hosting. Like an elevation of public access!!
```

### Using External Packages

We can add external packages for use in our projects by specifying them in our Cargo.toml
This tells Cargo to download the rand package and dependencies of the rand package from crates.io and then make rand package available to our project.

We then bring the rand package into scope (and the Rng trait)
```Rust
use rand::Rng;
```

This process is the same for all external packages. For std packages, we just must bring it into scope
```Rust
use std::collections::HashMap;
```

We can use nested paths to help with organizing imports
```Rust
use std::cmp::Ordering;
use std::io;
```
can be simplified into this:
```Rust
use std::{cmp::Ordering, io};
```
This can be done at any level during the path.
```Rust
use std::io;
use std::io::Write;
```
is the same as
```Rust
use std::io::{self, Write};
```

The glob operator for imports!!
Sometimes we just want ot bring all public items from a path into scope.
```Rust
use std::collections::*;
```
will bring all public items in collections module into scope. This operator is really commonly used for test modules!

### Separating modules into separate files
See restaurant3 package for details for this section.

The semicolon after the line
```Rust
mod front_of_house;
```
means that we will load the contents of a file with the same name as the module to define the front_of_house module. If we were to continue this and do the same with hosting module inside of front_of_house, we would make a directory for front_of_house as hosting file will be contained within that.

The module tree is completely the same, the function calls will remain the same as well.
Next up, Common collections!