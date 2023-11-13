# uDataTable
_NOTE: This crate is still in development and is not ready for use._

A rust library for creating an array of a generic type with a maximum capacity of rows. This crate is meant to be
used with the `ufmt` crate in a `no_std` environment. All data is saved on the stack, so no heap allocations ar
required. Column names are defined for the row type when the data table is created. The data table can be
appended to up to the maximum number of rows defined at compile time. The data table contents can be erased to
reset the length to zero.

## Example
### Create a data table and appending rows
The row type must implement the `Copy`, `Default`, `uDebug`, and `uDisplay` traits.
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
//!
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
```