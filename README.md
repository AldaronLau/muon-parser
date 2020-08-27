# Endec
Rust build-dependency for encoding and decoding file formats.  Experimental
alternative to serde focusing on faster link times (similar to `bindgen`).

## 1. Pick A Schema Format
The first thing you need to do is pick a schema format.  You can also choose not
to pick a schema (in which case the file is read into an `Any` type).  We'll use
a [MuON](https://github.com/muon-data/muon) schema.

Make a file called "schemas/MyNotes.muon"

```muon
:::
# A list of books
book: list record
    name: text
    author: text
    year: text
    character: list record
        name: text
        location: text
cycles: number >= 0.0
byte: int >=0 <256
:::
```

In your `build.rs` script:
```rust
fn main() {
    endec::file("schemas/MyNotes.muon");
}
```

In `src/endec.rs`:
```rust
include!(concat!(env!("OUT_DIR"), "/endec.rs"));
```

Now the macro will expand to:
```rust
/// Automatically-generated from schema `schemas/MyNotes.muon`
pub struct MyNotes_Book_Character {
    /// 
    pub name: String,
    /// 
    pub location: String,
}

/// Automatically-generated from schema `schemas/MyNotes.muon`
pub struct MyNotes_Book {
    /// 
    pub name: String,
    /// 
    pub author: String,
    /// 
    pub year: i32,
    /// 
    pub character: Vec<MyNotes_Book_Character>,

}

/// Automatically-generated from schema `schemas/MyNotes.muon`
pub struct MyNotes {
    /// A list of books
    pub book: Vec<MyNotesBook>,
    /// 
    pub cycles: f32,
    /// 
    pub byte: u8,
}

impl Default for MyNotes {
    fn default() -> Self {
        MyNotes::new()
    }
}

impl MyNotes {
    /// Initialize to default values specified in schema (or if not present,
    /// Rust's defined default)
    pub fn new() -> Self {
        // ...
    }

    /// Decode bytes into the struct.
    pub const fn parse(slice: &[u8]) -> Result<Self> {
        // ...
    }

    /// Read this format from a reader.
    pub fn read<R: Read>(reader: R) -> Result<Self> {
        // ...
    }

    /// Write this format into a writer.
    pub fn write<W: Write>(&self, writer: W) -> Result<()> {
        // ...
    }
    
    // ... (private functions)
}
```

Add a file to be read `src/example.muon`:
```muon
book: Pale Fire
  author: Vladimir Nabokov
  year: 1962
  character: John Shade
    location: New Wye
  character: Charles Kinbote
    location: Zembla
book: The Curious Incident of the Dog in the Night-Time
  author: Mark Haddon
  year: 2003
  character: Christopher Boone
    location: Swindon
  character: Siobhan
cycles: 1.5
byte: 42
```

Then in `main.rs`,

```
mod endec;

use endec::MyNotes;

const MY_NOTES: MyNotes = MyNotes::parse(include!("src/example.muon"));

fn main() {
    println!("{:?}", MY_NOTES);
}
```

It parses it at compile time!  If you want to use runtime for compression or
other reasons, use `read()` instead of `parse()`.  In this case the decoded data
is actually smaller, so it makes sense to use `parse()`.
