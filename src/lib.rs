//! Data collection Rust library for embedded systems, such as the Arduino.
//!
//! ## Overview
//! This library enables the creation of an array of a generic type with a maximum capacity of rows.
//! This crate is meant to be used with the [`ufmt`] crate in a `no_std` environment. It was specifically
//! created for sensor data collection on small microcrontrollers, such as the Arduino.
//!
//! All data is saved on the stack, so no heap allocations are required. Column names are defined for the row type
//! when the data table is created. The data table can be appended to up to the maximum number
//! of rows defined at compile time. The data table contents can be erased to reset the length to zero.
//!
//! A [`uDataTable`](crate::uDataTable) structure can be displayed with `ufmt` using the `uDisplay` or `uDebug` trait.
//! The intent is to use the `uDisplay` trait to print the data in a csv format and the `uDebug`
//! trait to print the headers and the length of the table. You must define the `uDisplay` and
//! `uDebug` traits for the row type if your row type is not a primitive type.
//!
//! The [`uDataTable`](crate::uDataTable) structure can also be plotted with the optional `plot` feature. The [`plot`](crate::uDataTable::plot) method
//! requires a function that takes a reference to the row type and returns an `i32`. The
//! `plot` method will plot the values returned by the function for each row in the data table.
//!
//! [`ufmt`]: https://crates.io/crates/ufmt
//!
//! ## Usage
//! Add the following to your `Cargo.toml` file to use the `udatatable` crate.
//! ```toml
//! [dependencies]
//! udatatable = "0.1"
//! ```
//! ### Features
//! * `plot` - Enables the [`plot`](crate::uDataTable::plot) method. This was made an option feature to allow you to keep your
//! code size small if you don't need the [`plot`](crate::uDataTable::plot) method.
//! ## Example
//! Create a data table, append rows, and display the contents. Note that the row type must
//! implement the `Copy`, `Default`, `uDebug`, and `uDisplay` traits.
//! ```rust
//! use ufmt::{uDebug, uDisplay, uWrite, uwrite, uwriteln, Formatter};
//! use udatatable::uDataTable;
//!
//! // Define the row type
//! #[derive(Copy, Clone, Default)]
//! struct Row {
//!     a: u32,
//!     b: u32,
//!     c: u32,
//! }
//!
//! // Define the uDisplay and uDebug traits for the row type
//! impl uDebug for Row {
//!     fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
//!     where
//!         W: uWrite + ?Sized,
//!     {
//!         uwrite!(f, "Row {{ a: {}, b: {}, c: {} }}", self.a, self.b, self.c)
//!     }
//! }
//!
//! impl uDisplay for Row {
//!     fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
//!     where
//!         W: uWrite + ?Sized,
//!     {
//!         // The uDisplay trait is meant to print the data in a csv format
//!         uwrite!(f, "{}, {}, {}", self.a, self.b, self.c)
//!     }
//! }
//!
//! // Create the data table
//! const N: usize = 10;
//! const M: usize = 3;
//! let mut table = uDataTable::<Row, N, M>::new(["a", "b", "c"]);
//!
//! // Append rows to the data table
//! for i in 0..5 {
//!     let row = Row {
//!         a: i as u32,
//!         b: i as u32 * 2,
//!         c: i as u32 * 3,
//!     };
//!     if let Err(error) = table.append(row) {
//!         // handle the error
//!     }
//! }
//!
//! assert_eq!(table.length(), 5);
//! assert_eq!(*table.headers(), ["a", "b", "c"]);
//! assert_eq!(table.get(0).unwrap().a, 0);
//! assert_eq!(table.get(0).unwrap().b, 0);
//! assert_eq!(table.get(0).unwrap().c, 0);
//! assert_eq!(table.get(1).unwrap().a, 1);
//! assert_eq!(table.get(1).unwrap().b, 2);
//! assert_eq!(table.get(1).unwrap().c, 3);
//! assert_eq!(table.get(2).unwrap().a, 2);
//! assert_eq!(table.get(2).unwrap().b, 4);
//! assert_eq!(table.get(2).unwrap().c, 6);
//!
//! // Display the data table
//! let mut s = String::new();
//! ufmt::uwrite!(&mut s, "{}", table).ok();
//! assert_eq!(s, "\"a\",\"b\",\"c\"\n0, 0, 0\n1, 2, 3\n2, 4, 6\n3, 6, 9\n4, 8, 12\n");
//!
//! // Display the data table with uDebug
//! let mut s = String::new();
//! ufmt::uwrite!(&mut s, "{:?}", table).ok();
//! assert_eq!(s, "uDataTable<[\"a\", \"b\", \"c\"], length: 5>");
//!
//! #[cfg(feature = "plot")]
//! {
//!     // graph the data table for value `a`
//!     let mut s = String::new();
//!     table.plot(&mut s, |row| row.a as i32);
//!     assert_eq!(s, "4 |    *\n  |   *.\n  |  *..\n  | *...\n0 |*....\n");
//! }
//! ```
//!
#![no_std]
use ufmt::{uDebug, uDisplay, uWrite, uwrite, uwriteln, Formatter};

/// The [`uDataTable`] structure.
/// # Generic Parameters
/// * `T` - The row type. This type must implement the `Copy`, `Default`, `uDebug`, and `uDisplay` traits.
/// * `N` - The maximum number of rows in the data table. This value must be greater than zero. Note that the
/// data table will be stored on the stack, so the maximum number of rows should be kept small.
/// * `M` - The number of columns in the data table, or more specifically, the number of column names that will be passed
/// to the `new` method's `headers` parameter. This value must be greater than zero.
/// # Fields
/// * `headers` - An array of `M` strings that are the column names for the data table.
/// * `data` - An array of `N` rows of type `T`.
/// * `length` - The number of rows of data that has been inserted into the able. This value will be between 0..N.
#[allow(non_camel_case_types)]
pub struct uDataTable<'a, T: Copy + Default + uDebug + uDisplay, const N: usize, const M: usize> {
    headers: [&'a str; M],
    data: [T; N],
    length: usize,
}

#[allow(dead_code)]
impl<'a, T: Copy + Default + uDebug + uDisplay, const N: usize, const M: usize>
    uDataTable<'a, T, N, M>
{
    /// Create a new data table with the specified headers passed in `headers`. There should be M headers in the passed `headers` array.
    pub fn new(headers: [&'a str; M]) -> Self {
        Self {
            headers,
            data: [T::default(); N],
            length: 0,
        }
    }

    /// Get a reference to the row at the specified `index`. The `index`` must be less than the length of the table.
    pub fn get(&self, index: usize) -> Result<&T, uDataTableError> {
        if index < self.length {
            Result::Ok(&self.data[index])
        } else {
            Result::Err(uDataTableError::RowIndexOutOfBounds)
        }
    }

    /// Append a row to the data table. The length of the table will be increased by one. The row
    /// will be copied into the data table. The row must implement the Copy trait. If the length
    /// of the table is equal to N, then the row will not be appended and an error will be returned.
    pub fn append(&mut self, row: T) -> Result<&T, uDataTableError> {
        if self.length < N {
            self.data[self.length] = row;
            self.length += 1;
            Result::Ok(&self.data[self.length - 1])
        } else {
            Result::Err(uDataTableError::CannotGrowTable)
        }
    }

    /// Erase the data table. The length of the table will be set to zero.
    pub fn erase(&mut self) {
        self.length = 0;
        for i in 0..N {
            self.data[i] = T::default();
        }
    }

    /// Get the length of the data table.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Get a reference to the headers of the data table.
    pub fn headers(&self) -> &[&'a str; M] {
        &self.headers
    }

    /// Plots the data table. The data table will be plotted with rows on the horizontal axis and values
    /// on the vertical axis. The plot method will scan thrugh all the rows in th data table
    /// with the passed `value` function to determine the range of values to be plotted. The plot method will then
    /// scale the values to fit in the display area. The plot method will display the range of values
    /// on the vertical axis and the row index on the horizontal axis.
    ///
    /// # Arguments
    ///
    /// * `f` - The `ufmt::uWrite` object that the grph should be printed to.
    /// * `value` - A function that gets called on each row in the data table to determine the value from that row to plot.
    /// This function must take a reference to the row type and return an `i32`. The mapping of the desired
    /// row value to the `i32` is for display purposes.
    #[cfg(any(plot, doc))]
    pub fn plot<W>(&self, f: &mut W, value: fn(&T) -> i32)
    where
        W: uWrite + ?Sized,
    {
        // first we need to scan through the data to find the range of
        // values that we need to plot
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for row in self.data.iter() {
            let value = value(row);
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
        }
        let min_digits = Self::count_digits(min);
        let max_digits = Self::count_digits(max);
        let digits = if min_digits > max_digits {
            min_digits
        } else {
            max_digits
        };

        // now we can calculate the scale factor
        let scale = 1.0 / (max - min) as f32;
        const MAX_HEIGHT: i32 = 23;
        let display_height = if (max - min) as i32 > MAX_HEIGHT {
            MAX_HEIGHT
        } else {
            (max - min) as i32
        };
        // now we can plot the data with rows on horizontal axis and values on vertical axis
        for h in (0..display_height + 1).rev() {
            if h == (display_height as f32 * (0 - min) as f32 / (max - min) as f32) as i32 {
                Self::write_n_spaces(digits - 1, f);
                uwrite!(f, "0 |").ok();
            } else if h == display_height {
                Self::write_n_spaces(digits - max_digits, f);
                uwrite!(f, "{} |", max).ok();
            } else if h == 0 {
                Self::write_n_spaces(digits - min_digits, f);
                uwrite!(f, "{} |", min).ok();
            } else {
                Self::write_n_spaces(digits, f);
                uwrite!(f, " |").ok();
            }
            for r in 0..self.length() {
                if let Result::Ok(row) = self.get(r) {
                    let value = value(row);
                    let scaled_value =
                        ((value - min) as f32 * scale * display_height as f32) as i32;
                    if scaled_value == h {
                        uwrite!(f, "*").ok();
                    } else if scaled_value > h {
                        uwrite!(f, ".").ok();
                    } else {
                        uwrite!(f, " ").ok();
                    }
                }
            }
            uwriteln!(f, "").ok();
        }
    }

    fn count_digits(value: i32) -> u32 {
        let mut n = value;
        let mut count = 0;
        if n < 0 {
            n = -n;
            count += 1; // for the '-' sign
        }
        loop {
            count += 1;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        count
    }

    #[cfg(feature = "plot")]
    fn write_n_spaces<W>(n: u32, f: &mut W)
    where
        W: uWrite + ?Sized,
    {
        for _ in 0..n {
            uwrite!(f, " ").ok();
        }
    }
}

impl<'a, T: Copy + Default + uDebug + uDisplay, const N: usize, const M: usize> uDebug
    for uDataTable<'a, T, N, M>
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uwrite!(f, "uDataTable<[\"")?;
        for i in 0..M {
            uwrite!(f, "{}", self.headers[i])?;
            if i < M - 1 {
                uwrite!(f, "\", \"")?;
            }
        }
        uwrite!(f, "\"], length: {}>", self.length)?;
        Result::Ok(())
    }
}

impl<'a, T: Copy + Default + uDebug + uDisplay, const N: usize, const M: usize> uDisplay
    for uDataTable<'a, T, N, M>
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        for i in 0..M {
            uwrite!(f, "\"{}\"", self.headers[i])?;
            if i < M - 1 {
                uwrite!(f, ",")?;
            }
        }
        uwrite!(f, "\n")?;
        for i in 0..self.length {
            uwriteln!(f, "{}", self.data[i])?;
        }
        Ok(())
    }
}

/// Errors that can occur when using the uDataTable structure.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum uDataTableError {
    /// The passed row index is out of bounds.
    RowIndexOutOfBounds,
    /// The data table cannot grow any larger.
    CannotGrowTable,
}

impl uDebug for uDataTableError {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            uDataTableError::RowIndexOutOfBounds => uwrite!(f, "RowIndexOutOfBounds"),
            uDataTableError::CannotGrowTable => uwrite!(f, "CannotGrowTable"),
        }
    }
}

impl uDisplay for uDataTableError {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        uDebug::fmt(self, f)
    }
}
