// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use super::{Format, FormattedValueIntrinsic, FormattedValueNoneBuilder};
use crate::utils::Displayable;

// rustdoc-stripper-ignore-next
/// A signed wrapper.
///
/// This wrapper allows representing a signed value from a type
/// which is originaly unsigned. In C APIs, this is represented
/// by a tuple with a signed integer positive or negative and
/// the absolute value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Signed<T> {
    Negative(T),
    Positive(T),
}

impl<T> Signed<T> {
    pub fn is_positive(self) -> bool {
        matches!(self, Signed::Positive(_))
    }

    // rustdoc-stripper-ignore-next
    /// Returns `Some(value)`, where `value` is the inner value,
    /// if `self` is positive.
    pub fn positive(self) -> Option<T> {
        match self {
            Signed::Positive(val) => Some(val),
            Signed::Negative(_) => None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Positive(v)` to `Ok(v)` and `Negative(_)` to `Err(err)`.
    pub fn positive_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Signed::Positive(val) => Ok(val),
            Signed::Negative(_) => Err(err),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Positive(v)` to `Ok(v)` and `Negative(v)` to `Err(err(v))`.
    pub fn positive_or_else<E, F: FnOnce(T) -> E>(self, err: F) -> Result<T, E> {
        match self {
            Signed::Positive(val) => Ok(val),
            Signed::Negative(val) => Err(err(val)),
        }
    }

    pub fn is_negative(self) -> bool {
        matches!(self, Signed::Negative(_))
    }

    // rustdoc-stripper-ignore-next
    /// Returns `Some(value)`, where `value` is the inner value,
    /// if `self` is negative.
    pub fn negative(self) -> Option<T> {
        match self {
            Signed::Negative(val) => Some(val),
            Signed::Positive(_) => None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Negative(v)` to `Ok(v)` and `Positive(_)` to `Err(err)`.
    pub fn negative_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Signed::Negative(val) => Ok(val),
            Signed::Positive(_) => Err(err),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transforms the `Signed<T>` into a `Result<T, E>`,
    /// mapping `Negative(v)` to `Ok(v)` and `Positive(_)` to `Err(err(v))`.
    pub fn negative_or_else<E, F: FnOnce(T) -> E>(self, err: F) -> Result<T, E> {
        match self {
            Signed::Negative(val) => Ok(val),
            Signed::Positive(val) => Err(err(val)),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the multiplication factor for this `Signed`.
    ///
    /// Returns:
    ///
    /// - `1` if the value must be considered as positive.
    /// - `-1` if the value must be considered as negative.
    pub fn factor(self) -> i32 {
        match self {
            Signed::Positive(_) => 1i32,
            Signed::Negative(_) => -1i32,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the absolute value of `self`.
    pub fn abs(self) -> T {
        match self {
            Signed::Positive(val) | Signed::Negative(val) => val,
        }
    }
}

impl<T> std::ops::Neg for Signed<T> {
    type Output = Signed<T>;

    fn neg(self) -> Self {
        match self {
            Signed::Positive(val) => Signed::Negative(val),
            Signed::Negative(val) => Signed::Positive(val),
        }
    }
}

impl<T> fmt::Display for Signed<T>
where
    T: fmt::Display + FormattedValueIntrinsic,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        let (sign, val) = match self {
            Signed::Positive(val) => ('+', val),
            Signed::Negative(val) => ('-', val),
        };

        f.write_char(sign)?;
        fmt::Display::fmt(&val, f)
    }
}

impl<T> Displayable for Signed<T>
where
    T: fmt::Display + FormattedValueIntrinsic,
{
    type DisplayImpl = Signed<T>;

    fn display(self) -> Self::DisplayImpl {
        self
    }
}

impl<T> Signed<Option<T>> {
    // rustdoc-stripper-ignore-next
    /// Transposes a `Signed` `Option` into an `Option` of a `Signed`.
    ///
    /// Note that if the inner value was `None`, the sign is lost.
    pub fn transpose(self) -> Option<Signed<T>> {
        use Signed::*;

        match self {
            Positive(Some(val)) => Some(Positive(val)),
            Negative(Some(val)) => Some(Negative(val)),
            _ => None,
        }
    }
}

pub struct DisplayableOptionSigned<T>(Option<Signed<T>>);

impl<T> fmt::Display for DisplayableOptionSigned<T>
where
    T: fmt::Display + FormattedValueIntrinsic,
    Option<T>: Displayable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref signed) => fmt::Display::fmt(signed, f),
            None => fmt::Display::fmt(&Option::<T>::None.display(), f),
        }
    }
}

impl<T> Displayable for Option<Signed<T>>
where
    T: fmt::Display + FormattedValueIntrinsic,
    Option<T>: Displayable,
{
    type DisplayImpl = DisplayableOptionSigned<T>;

    fn display(self) -> Self::DisplayImpl {
        DisplayableOptionSigned(self)
    }
}

impl<T> Displayable for Signed<Option<T>>
where
    T: fmt::Display + FormattedValueIntrinsic,
    Option<T>: Displayable,
{
    type DisplayImpl = DisplayableOptionSigned<T>;

    fn display(self) -> Self::DisplayImpl {
        DisplayableOptionSigned(self.transpose())
    }
}

// rustdoc-stripper-ignore-next
/// A trait implemented on unsigned types which can be converted into [`crate::Signed`]s.
pub trait UnsignedIntoSigned: Copy + Sized {
    type Signed;

    // rustdoc-stripper-ignore-next
    /// Converts `self` into a `Signed` matching the given `sign`.
    fn into_signed(self, sign: i32) -> Self::Signed {
        if sign.is_positive() {
            self.into_positive()
        } else {
            self.into_negative()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Converts `self` into a `Signed::Positive`.
    fn into_positive(self) -> Self::Signed;

    // rustdoc-stripper-ignore-next
    /// Converts `self` into a `Signed::Negative`.
    fn into_negative(self) -> Self::Signed;
}

impl_unsigned_int_into_signed!(u64);
impl_signed_ops!(u64);
impl_signed_div_mul!(u64);

impl_unsigned_int_into_signed!(u32);
impl_signed_ops!(u32);
impl_signed_div_mul!(u32);

impl From<i64> for Signed<u64> {
    fn from(val: i64) -> Signed<u64> {
        skip_assert_initialized!();
        match val {
            positive if positive.is_positive() => Signed::Positive(positive as u64),
            i64::MIN => {
                // `i64::MIN.abs()` can't be represented as an `i64`
                Signed::Negative((-(i64::MIN as i128)) as u64)
            }
            negative => Signed::Negative((-negative) as u64),
        }
    }
}

impl From<isize> for Signed<usize> {
    fn from(val: isize) -> Signed<usize> {
        skip_assert_initialized!();
        match val {
            positive if positive.is_positive() => Signed::Positive(positive as usize),
            isize::MIN => {
                // `isize::MIN.abs()` can't be represented as an `isize`
                Signed::Negative((-(isize::MIN as i128)) as usize)
            }
            negative => Signed::Negative((-negative) as usize),
        }
    }
}

// `i32::MIN.abs()` can't be represented as an `i32`
impl From<i32> for Signed<u32> {
    fn from(val: i32) -> Signed<u32> {
        skip_assert_initialized!();
        if val.is_positive() {
            Signed::Positive(val as u32)
        } else {
            Signed::Negative((-(val as i64)) as u32)
        }
    }
}

pub trait NoneSignedBuilder: FormattedValueNoneBuilder {
    type Signed;

    // rustdoc-stripper-ignore-next
    /// Returns the `None` value for `Self` as a `Signed<FullRange>` if such a value can be represented.
    ///
    /// See details in [`FormattedValueNoneBuilder::none`].
    ///
    /// # Panics
    ///
    /// Panics if `Self` is `GenericFormattedValue` in which case, the `Format` must be known.
    fn none_signed() -> Self::Signed;

    // rustdoc-stripper-ignore-next
    /// Returns the `None` value for `Self` as a `Signed<FullRange>`, if such a value can be represented.
    ///
    /// See details in [`FormattedValueNoneBuilder::none_for_format`].
    ///
    /// # Panics
    ///
    /// Panics if `None` can't be represented by `Self` for `format` or by the requested
    /// `GenericFormattedValue` variant.
    fn none_signed_for_format(format: Format) -> Self::Signed;
}

impl<T> NoneSignedBuilder for T
where
    T: UnsignedIntoSigned + FormattedValueNoneBuilder,
{
    type Signed = <T as UnsignedIntoSigned>::Signed;

    fn none_signed() -> Self::Signed {
        Self::none().into_positive()
    }

    fn none_signed_for_format(format: Format) -> Self::Signed {
        skip_assert_initialized!();
        Self::none_for_format(format).into_positive()
    }
}