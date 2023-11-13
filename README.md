# µDataTable
_NOTE: This crate is still in development and is not ready for general use._

An array of a generic type with a maximum capacity of rows. This crate is meant to be
used with the `ufmt` crate in a `no_std` environment. All data is saved on the
stack, so no heap allocations are required. Column names are defined for the row type
when the data table is created. The data table can be appended to up to the maximum number
of rows defined at compile time. The data table contents can be erased to reset the length to zero.

A `uDataTable` structure can be displayed with `ufmt` using the `uDisplay` or `uDebug` trait.
The intent is to use the `uDisplay` trait to print the data in a csv format and the `uDebug`
trait to print the headers and the length of the table. You must define the `uDisplay` and
`uDebug` traits for the row type if your row type is not a primitive type.

The `uDataTable` structure can also be plotted with the `plot` method. The `plot` method
requires a function that takes a reference to the row type and returns an `i32`. The
`plot` method will plot the values returned by the function for each row in the data table.
