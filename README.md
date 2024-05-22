This is a Rust crate providing fixed-point arithmetic with _statically verified_
overflow safety and bit shift correctness.

Please note: this crate requires nightly Rust, for the `generic_const_exprs` feature.
In addition, this is an "alpha" release with incomplete documentation and incomplete
test coverage.

This crate provides fixed-point arithmetic with _statically verified_
overflow safety and bit shift correctness.
Fixed-point arithmetic represents fractional values as integers with
an implicit bit shift.  For example, the decimal number 2.375 (in base 2: 10.011) could
be represented in fixed-point as the integer `0b10011` (decimal 19) with an implicit bit shift of 3.  It is
the programmer's responsibility to keep track of all the bit shifts used in a program,
ensure they are consistent with each other, and avoid any overflows during
arithmetic operations.

In contrast, floating-point numbers automatically adjust the "bit shift" (i.e. the exponent)
to provide the largest possible resolution which will not overflow.
They are easy to use, and they do the right thing most of
the time.  However, they can cause subtle rounding bugs which are famously difficult
to identify and prevent.  In the immortal words of Professor Gerald Sussman,
"Nothing brings fear to my heart more than a floating-point number."

This crate uses the Rust type system to provide fixed-point numbers with compile-time
bit shift checking and overflow protection.  Each fixed-point type has two const generic
parameters, one describing the bit shift and one describing the maximum
number of bits which could be nonzero.  Each arithmetic operation is implemented with
an output type which correctly reflects the bits and shift of the result.  For example,
the result of multiplying a 10-bit number (shifted by 2) and a 12-bit number (shifted by 3)
is a 22-bit number (shifted by 5).

The trait `Fp` represents any fixed-point number stored as an
integer, and the structs `FpXxx<const BITS: u32, const SHIFT: i32>` implement the
`Fp` trait for each integer type `Xxx`.  Arithmetic operations on the fixed-point
types are guaranteed to provide correctness and overflow safety with zero runtime
overhead.