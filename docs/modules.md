source: https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file

Modules are important to understand, but I find most documentations often leave you scratching your head on that matter.

# Coming from Python or Javascript?

Roughly, mod.rs is kind of like __init__.py in python or index.js in Javascript. But only kind of. This is a bit more complicated in Rust.

# Rust is different

Folders are not immediately ready to use as modules in Rust.

You have to add a file named mod.rs in a folder to expose a new module named like that folder. The code in mod.rs is the content of that module. All other files in the folder may in turn be exposed as submodules (more on that below).

# Wait, there is another way

Instead of adding a mod.rs file in the folder, you may add a file named <folder_name>.rs at the same level as the folder.

As noted by MarkusToman in the comments, this is the preferred way since rustc 1.30.

From the [Rust reference](https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames):

```
Note: Previous to rustc 1.30, using mod.rs files was the way to load a module with nested children. It is encouraged to use the new naming convention as it is more consistent, and avoids having many files named mod.rs within a project.
```

# Complete example

```
src
    utils
        bar.rs
        foo.rs
    main.rs
```

At this point, the compiler doesn't know about ```src/utils/foo.rs``` and ```src/utils/bar.rs```.

First, you must expose ```src/utils/```. As seen above, you have 2 options:
* add the file: ```src/utils/mod.rs```
* add the file ```src/utils.rs``` (named exactly like the folder, without the extension)

Now, relative to the src folder (aka the crate level), a module named ```utils``` is available.

Second, you must expose the files ```src/utils/foo.rs``` and ```src/utils/bar.rs```.

To do that, the ```utils``` module must declare 2 new submodules named after these files. So, the content of ```src/utils/mod.rs``` (or ```src/utils.rs```) should be:

```rust
pub mod bar;
pub mod foo;
```

Now whatever is public in those 2 files is available in other modules! 🎉

And you may write the following in ```src/main.rs```:

```rust
mod utils;
use utils::{foo, bar};
```

# Resulting file structure

## Option 1 • mod.rs (the old way):

```
src
    utils
        bar.rs
        foo.rs
        mod.rs
    main.rs
```

## Option 2 • <folder_name>.rs (the preferred way):

```
src
    utils
        bar.rs
        foo.rs
    utils.rs
    main.rs
```




