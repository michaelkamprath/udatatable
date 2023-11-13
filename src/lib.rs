//! # uDataTable
//! An array of a generic type with a maximum capacity of rows. This crate is meant to be
//! used with the `ufmt` crate in a `no_std` environment. All data is saved on the
//! stack, so no heap allocations are required. Column names are defined for the row type
//! when the data table is created. The data table can be appended to up to the maximum number
//! of rows defined at compile time. The data table contents can be erased to reset the length to zero.
//!
//! A `uDataTable` structure can be displayed with `ufmt` using the `uDisplay` or `uDebug` trait.
//! The intent is to use the `uDisplay` trait to print the data in a csv format and the `uDebug`
//! trait to print the headers and the length of the table. You must define the `uDisplay` and
//! `uDebug` traits for the row type if your row type is not a primitive type.
//!
//! The `uDataTable` structure can also be plotted with the `plot` method. The `plot` method
//! requires a function that takes a reference to the row type and returns an `i32`. The
//! `plot` method will plot the values returned by the function for each row in the data table.
//!
//! ## Example
//! ### Create a data table and appending rows
//! The row type must implement the `Copy`, `Default`, `uDebug`, and `uDisplay` traits.
//! ```
//! use ufmt;
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
//! ```
#![no_std]
use ufmt::{uDebug, uDisplay, uWrite, uwrite, uwriteln, Formatter};

/// N rows of M columns.
/// The debug and display implementations are meant to be used with the ufmt crate. The debug
/// implementation will print the headers and the length of the table. The display implementation
/// will print the table in a csv format.
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
    pub fn new(headers: [&'a str; M]) -> Self {
        Self {
            headers,
            data: [T::default(); N],
            length: 0,
        }
    }

    pub fn get(&self, index: usize) -> Result<&T, uDataTableError> {
        if index < self.length {
            Result::Ok(&self.data[index])
        } else {
            Result::Err(uDataTableError::RowIndexOutOfBounds)
        }
    }

    pub fn append(&mut self, row: T) -> Result<&T, uDataTableError> {
        if self.length < N {
            self.data[self.length] = row;
            self.length += 1;
            Result::Ok(&self.data[self.length - 1])
        } else {
            Result::Err(uDataTableError::CannotGrowTable)
        }
    }

    pub fn erase(&mut self) {
        self.length = 0;
        for i in 0..N {
            self.data[i] = T::default();
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn headers(&self) -> &[&'a str; M] {
        &self.headers
    }

    pub fn plot<W>(
        &self,
        f: &mut W,
        value: fn(&T) -> i32,
    ) where
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
        const HEIGHT: i32 = 23;
        // now we can plot the data with rows on horizontal axis and values on vertical axis
        for h in (0..HEIGHT+1).rev() {
            if h == (HEIGHT as f32 * (0 - min) as f32 / (max - min) as f32) as i32 {
                Self::write_n_spaces(digits-1, f);
                uwrite!(f, "0 |").ok();
            } else if h == HEIGHT {
                Self::write_n_spaces(digits-max_digits, f);
                uwrite!(f, "{} |", max).ok();
            } else if h == 0 {
                Self::write_n_spaces(digits-min_digits, f);
                uwrite!(f, "{} |", min).ok();
            } else {
                Self::write_n_spaces(digits, f);
                uwrite!(f, " |").ok();
            }
            for r in 0..self.length() {
                if let Result::Ok(row) = self.get(r) {
                    let value = value(row);
                    let scaled_value = ((value - min) as f32 * scale * HEIGHT as f32) as i32;
                    if scaled_value == h {
                        uwrite!(f, "*").ok();
                    } else if scaled_value > h {
                        uwrite!(f, ".").ok();
                    } else {
                        uwrite!(f, " ").ok();
                    }
                }
            }
            uwriteln!(f,"").ok();
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

    fn write_n_spaces<W>(n: u32,  f: &mut W)
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

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum uDataTableError {
    RowIndexOutOfBounds,
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
