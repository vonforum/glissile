#[cfg(feature = "serde")]
mod serde_impl {
	use serde::{Deserialize, Serialize};

	use super::Fx32Var;

	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(untagged)]
	pub enum Fx32VarSerde {
		Raw(i64),
		Float(f32),
	}

	impl<const N: i64> From<Fx32VarSerde> for Fx32Var<N> {
		fn from(value: Fx32VarSerde) -> Self {
			match value {
				Fx32VarSerde::Float(value) => Fx32Var::<N>::from(value),
				Fx32VarSerde::Raw(value) => Fx32Var::<N>::from_raw(value),
			}
		}
	}

	impl<const N: i64> From<Fx32Var<N>> for Fx32VarSerde {
		fn from(value: Fx32Var<N>) -> Self {
			Fx32VarSerde::Raw(value.internal)
		}
	}
}

use core::ops::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_impl::Fx32VarSerde;

use crate::{DEFAULT_RESOLUTION, util::*};

/// 32-bit fixed-point number
/// N is the scaling factor
/// Internally stored as a 64-bit integer to avoid overflow
#[derive(Clone, Copy, PartialEq, Default, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "Fx32VarSerde", into = "Fx32VarSerde"))]
pub struct Fx32Var<const N: i64 = DEFAULT_RESOLUTION> {
	pub internal: i64,
}

impl<const N: i64> core::fmt::Debug for Fx32Var<N> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Fx32Var({})", self.as_f32())
	}
}

impl<const N: i64> Fx32Var<N> {
	pub const ZERO: Self = Self::from_raw(0);
	pub const ONE: Self = Self::from_raw(N);

	pub const MIN: Self = Self::from_raw(i32::MIN as i64);
	pub const MAX: Self = Self::from_raw(i32::MAX as i64);

	pub const EPSILON: Self = Self::from_raw(1);

	#[inline(always)]
	pub const fn resolution() -> i64 {
		N
	}

	#[inline(always)]
	pub const fn new(value: i32) -> Self {
		Self::from_raw(value as i64 * N)
	}

	#[inline(always)]
	pub const fn from_raw(internal: i64) -> Self {
		Self { internal }
	}

	#[inline(always)]
	pub const fn from_parts(integral: i32, fractional: i32) -> Self {
		Self::from_raw((integral as i64 * N) + fractional as i64)
	}

	#[inline(always)]
	pub const fn raw(self) -> i64 {
		self.internal
	}

	#[inline]
	pub const fn sqrt(self) -> Self {
		Self::from_raw((self.internal * N).isqrt())
	}

	#[inline]
	pub fn recip(self) -> Option<Self> {
		if self.internal == 0 {
			None
		} else {
			Some(Self::from_raw(N * N / self.internal))
		}
	}

	#[inline]
	pub fn as_f32(&self) -> f32 {
		(*self).into()
	}

	#[inline]
	pub fn as_i32(&self) -> i32 {
		(*self).into()
	}
}

impl<const N: i64> From<f32> for Fx32Var<N> {
	#[inline]
	fn from(value: f32) -> Self {
		Self {
			internal: (value * N as f32) as i64,
		}
	}
}

impl<const N: i64> From<Fx32Var<N>> for f32 {
	#[inline]
	fn from(value: Fx32Var<N>) -> Self {
		value.internal as f32 / N as f32
	}
}

impl<const N: i64> From<i32> for Fx32Var<N> {
	#[inline]
	fn from(value: i32) -> Self {
		Self::new(value)
	}
}

impl<const N: i64> From<Fx32Var<N>> for i32 {
	#[inline]
	fn from(value: Fx32Var<N>) -> Self {
		(value.internal / N) as i32
	}
}

impl<const N: i64> Add for Fx32Var<N> {
	type Output = Self;

	#[inline]
	fn add(self, other: Self) -> Self {
		Self::from_raw(self.internal + other.internal)
	}
}

impl_ref!(Fx32Var<N>, Add, add);

impl<const N: i64> Sub for Fx32Var<N> {
	type Output = Self;

	#[inline]
	fn sub(self, other: Self) -> Self {
		Self::from_raw(self.internal - other.internal)
	}
}

impl_ref!(Fx32Var<N>, Sub, sub);

impl<const N: i64> Mul for Fx32Var<N> {
	type Output = Self;

	#[inline]
	fn mul(self, other: Self) -> Self {
		Self::from_raw((self.internal * other.internal) / N)
	}
}

impl_ref!(Fx32Var<N>, Mul, mul);

impl<const N: i64> Div for Fx32Var<N> {
	type Output = Self;

	#[inline]
	fn div(self, other: Self) -> Self {
		Self::from_raw((self.internal * N) / other.internal)
	}
}

impl_ref!(Fx32Var<N>, Div, div);

impl<const N: i64> Neg for Fx32Var<N> {
	type Output = Self;

	#[inline]
	fn neg(self) -> Self {
		Self::from_raw(self.internal.neg())
	}
}

impl<const N: i64> Neg for &Fx32Var<N> {
	type Output = Fx32Var<N>;

	#[inline]
	fn neg(self) -> Fx32Var<N> {
		(*self).neg()
	}
}

impl_assign!(Fx32Var<N>, Fx32Var<N>, add, AddAssign, add_assign);
impl_assign!(Fx32Var<N>, Fx32Var<N>, sub, SubAssign, sub_assign);
impl_assign!(Fx32Var<N>, Fx32Var<N>, mul, MulAssign, mul_assign);
impl_assign!(Fx32Var<N>, Fx32Var<N>, div, DivAssign, div_assign);

pub trait AsFx32<const N: i64> {
	fn as_fx32(&self) -> Fx32Var<N>;
}

impl<const N: i64> AsFx32<N> for f32 {
	fn as_fx32(&self) -> Fx32Var<N> {
		Fx32Var::from(*self)
	}
}

impl<const N: i64> AsFx32<N> for i32 {
	fn as_fx32(&self) -> Fx32Var<N> {
		Fx32Var::from(*self)
	}
}
