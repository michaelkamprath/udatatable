# µDataTable

<!-- cargo-sync-readme start -->

Data collection Rust library for embedded systems, such as the Arduino.

## Overview
This library enables the creation of an array of a generic type with a maximum capacity of rows.
This crate is meant to be used with the [`ufmt`] crate in a `no_std` environment. It was specifically
created for sensor data collection on small microcrontrollers, such as the Arduino.

All data is saved on the stack, so no heap allocations are required. Column names are defined for the row type
when the data table is created. The data table can be appended to up to the maximum number
of rows defined at compile time. The data table contents can be erased to reset the length to zero.

A [`uDataTable`](https://docs.rs/udatatable/latest/udatatable/struct.uDataTable.html) structure can be displayed with `ufmt` using the `uDisplay` or `uDebug` trait.
The intent is to use the `uDisplay` trait to print the data in a csv format and the `uDebug`
trait to print the headers and the length of the table. You must define the `uDisplay` and
`uDebug` traits for the row type if your row type is not a primitive type.

The [`uDataTable`](https://docs.rs/udatatable/latest/udatatable/struct.uDataTable.html) structure can also be plotted with the optional `plot` feature. The [`plot`](crate::uDataTable::plot) method
requires a function that takes a reference to the row type and returns an `i32`. The
`plot` method will plot the values returned by the function for each row in the data table.

[`ufmt`]: https://crates.io/crates/ufmt

## Usage
Add the following to your `Cargo.toml` file to use the `udatatable` crate.
```toml
[dependencies]
udatatable = "0.1"
```
### Features
* `plot` - Enables the [`plot`](crate::uDataTable::plot) method. This was made an option feature to allow you to keep your
code size small if you don't need the [`plot`](crate::uDataTable::plot) method.
## Example
Create a data table, append rows, and display the contents. Note that the row type must
implement the `Copy`, `Default`, `uDebug`, and `uDisplay` traits.
```rust
use ufmt::{uDebug, uDisplay, uWrite, uwrite, uwriteln, Formatter};
use udatatable::uDataTable;

// Define the row type
#[derive(Copy, Clone, Default)]
struct Row {
    a: u32,
    b: u32,
    c: u32,
}

// Define the uDisplay and uDebug traits for the row type
impl uDebug for Row {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "Row {{ a: {}, b: {}, c: {} }}", self.a, self.b, self.c)
    }
}

impl uDisplay for Row {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        // The uDisplay trait is meant to print the data in a csv format
        uwrite!(f, "{}, {}, {}", self.a, self.b, self.c)
    }
}

// Create the data table
const N: usize = 10;
const M: usize = 3;
let mut table = uDataTable::<Row, N, M>::new(["a", "b", "c"]);

// Append rows to the data table
for i in 0..5 {
    let row = Row {
        a: i as u32,
        b: i as u32 * 2,
        c: i as u32 * 3,
    };
    if let Err(error) = table.append(row) {
        // handle the error
    }
}

assert_eq!(table.length(), 5);
assert_eq!(*table.headers(), ["a", "b", "c"]);
assert_eq!(table.get(0).unwrap().a, 0);
assert_eq!(table.get(0).unwrap().b, 0);
assert_eq!(table.get(0).unwrap().c, 0);
assert_eq!(table.get(1).unwrap().a, 1);
assert_eq!(table.get(1).unwrap().b, 2);
assert_eq!(table.get(1).unwrap().c, 3);
assert_eq!(table.get(2).unwrap().a, 2);
assert_eq!(table.get(2).unwrap().b, 4);
assert_eq!(table.get(2).unwrap().c, 6);

// Display the data table
let mut s = String::new();
ufmt::uwrite!(&mut s, "{}", table).ok();
assert_eq!(s, "\"a\",\"b\",\"c\"\n0, 0, 0\n1, 2, 3\n2, 4, 6\n3, 6, 9\n4, 8, 12\n");

// Display the data table with uDebug
let mut s = String::new();
ufmt::uwrite!(&mut s, "{:?}", table).ok();
assert_eq!(s, "uDataTable<[\"a\", \"b\", \"c\"], length: 5>");

#[cfg(feature = "plot")]
{
    // graph the data table for value `a`
    let mut s = String::new();
    table.plot(&mut s, |row| row.a as i32);
    assert_eq!(s, "4 |    *\n  |   *.\n  |  *..\n  | *...\n0 |*....\n");
}
```


<!-- cargo-sync-readme end -->

## License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

Pull requests are welcome. When creating a pull request, please make sure that tests are included and passing. To run the tests, run `cargo test` from the root directory of the repository. Also, please be mindful to the intended embedded use case of this crate and keep the code size small.