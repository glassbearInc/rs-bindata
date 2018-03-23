[![](https://meritbadge.herokuapp.com/bindata)](https://crates.io/crates/bindata)

## bindata

------------------------------------------------------------------------------

`bindata` is a rust macro which loads files into your rust binary at compile
time. You can use this to embed binary assets into your executable. This macro
works on the stable release channel, but makes use of a custom derive macro
behind the scenes, so it definitely won't work on rustc's prior to 1.15. It has
only been tested so far on rustc 1.24.1.


## Installation

This crate compiles on the stable version of the compiler. Add it to the
dependencies in your `Cargo.toml` as so:

```
[dependencies]
bindata = "0.1.1"
bindata-impl = "0.1.0"
```

## Usage

```
#[macro_use]
extern crate bindata;
#[macro_use]
extern crate bindata_impl;

mod assets {
    bindata!("tests/data/a"); // A relative path from your package root.
}

fn main() {
    if let Some(asset) = assets::get("tests/data/a/georgie-porgie") {
        assert_eq!(Ok("pudding and pie\n"), str::from_utf8(&asset));
    }
}
```

You can specify multiple asset directories as well:

```
#[macro_use]
extern crate bindata;
#[macro_use]
extern crate bindata_impl;

mod assets {
    bindata!("tests/data/a", "tests/data/b"); // A relative path from your package root.
}

fn main() {
    if let Some(asset) = assets::get("tests/data/a/georgie-porgie") {
        assert_eq!(Ok("pudding and pie\n"), str::from_utf8(&asset));
    }
}
```
