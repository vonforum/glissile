use core::ops::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{DEFAULT_RESOLUTION, num::*, util::*};

macro_rules! strip_plus {
    {+ $($rest:tt)* } => { $($rest)* }
}

#[cfg(feature = "serde")]
type FxTuple2<const N: i64> = (Fx32Var<N>, Fx32Var<N>);
#[cfg(feature = "serde")]
type FxTuple3<const N: i64> = (Fx32Var<N>, Fx32Var<N>, Fx32Var<N>);
#[cfg(feature = "serde")]
type FxTuple4<const N: i64> = (Fx32Var<N>, Fx32Var<N>, Fx32Var<N>, Fx32Var<N>);

macro_rules! impl_vec {
	($name: ident, $tuple: expr, $($axis: ident),+) => {
		#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
		#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
		#[cfg_attr(feature = "serde", serde(from = $tuple, into = $tuple))]
		pub struct $name<const N: i64 = DEFAULT_RESOLUTION> {
			$(pub $axis: Fx32Var<N>,)+
		}

		impl<const N: i64> $name<N> {
			pub const ZERO: Self = Self::splat(Fx32Var::ZERO);
			pub const ONE: Self = Self::splat(Fx32Var::ONE);

			pub const MIN: Self = Self::splat(Fx32Var::MIN);
			pub const MAX: Self = Self::splat(Fx32Var::MAX);

			#[inline(always)]
			pub const fn new($($axis: Fx32Var<N>,)+) -> Self {
				Self { $($axis,)+ }
			}

			#[inline(always)]
			pub const fn splat(value: Fx32Var<N>) -> Self {
				Self { $($axis: value,)+ }
			}

			#[inline(always)]
			pub const fn from_raw($($axis: i64,)+) -> Self {
				Self { $($axis: Fx32Var::from_raw($axis),)+ }
			}

			#[inline]
			pub fn dot(self, rhs: Self) -> Fx32Var<N> {
				strip_plus!($(+ (self.$axis * rhs.$axis))+)
			}

			#[inline]
			pub fn length(self) -> Fx32Var<N> {
				self.dot(self).sqrt()
			}

			#[inline]
			pub fn length_recip(self) -> Option<Fx32Var<N>> {
				self.length().recip()
			}

			#[inline]
			pub fn normalize_or(self, fallback: Self) -> Self {
				let rcp = self.length_recip();
				if let Some(rcp) = rcp {
					if rcp != Fx32Var::<N>::ZERO {
						return self * rcp;
					}
				}

				fallback
			}

			#[inline]
			pub fn normalize_or_zero(self) -> Self {
				self.normalize_or(Self::ZERO)
			}
		}
	};
}

impl_vec!(FxVecVar2, "FxTuple2<N>", x, y);
impl_vec!(FxVecVar3, "FxTuple3<N>", x, y, z);
impl_vec!(FxVecVar4, "FxTuple4<N>", x, y, z, w);

macro_rules! impl_index {
	($name: ty, $($index: literal, $axis: ident),+) => {
		impl<const N: i64> Index<usize> for $name {
			type Output = Fx32Var<N>;

			#[inline]
			fn index(&self, index: usize) -> &Self::Output {
				match index {
					$($index => &self.$axis,)+
					_ => panic!("Index out of bounds"),
				}
			}
		}

		impl<const N: i64> IndexMut<usize> for $name {
			#[inline]
			fn index_mut(&mut self, index: usize) -> &mut Self::Output {
				match index {
					$($index => &mut self.$axis,)+
					_ => panic!("Index out of bounds"),
				}
			}
		}
	};
}

impl_index!(FxVecVar2<N>, 0, x, 1, y);
impl_index!(FxVecVar3<N>, 0, x, 1, y, 2, z);
impl_index!(FxVecVar4<N>, 0, x, 1, y, 2, z, 3, w);

macro_rules! impl_vec_op {
	($name: ty, $op: ident, $op_fn: ident, $($axis: ident),+) => {
		impl<const N: i64> $op for $name {
			type Output = Self;

			#[inline]
			fn $op_fn(self, rhs: Self) -> Self {
				Self {
					$($axis: self.$axis.$op_fn(rhs.$axis),)+
				}
			}
		}
	}
}

macro_rules! impl_vec_ops {
	($name: ty, $($axis:ident),+) => {
		impl_vec_op!($name, Add, add, $($axis),+);
		impl_ref!($name, Add, add);
		impl_assign!($name, $name, add, AddAssign, add_assign);

		impl_vec_op!($name, Sub, sub, $($axis),+);
		impl_ref!($name, Sub, sub);
		impl_assign!($name, $name, sub, SubAssign, sub_assign);

		impl_vec_op!($name, Mul, mul, $($axis),+);
		impl_ref!($name, Mul, mul);
		impl_assign!($name, $name, mul, MulAssign, mul_assign);

		impl_vec_op!($name, Div, div, $($axis),+);
		impl_ref!($name, Div, div);
		impl_assign!($name, $name, div, DivAssign, div_assign);
	};
}

impl_vec_ops!(FxVecVar2<N>, x, y);
impl_vec_ops!(FxVecVar3<N>, x, y, z);
impl_vec_ops!(FxVecVar4<N>, x, y, z, w);

macro_rules! impl_op {
	($name: ty, $op: ident, $op_fn: ident, $($axis: ident),+) => {
		impl<const N: i64> $op<Fx32Var<N>> for $name {
			type Output = $name;

			#[inline]
			fn $op_fn(self, rhs: Fx32Var<N>) -> $name {
				<$name>::new($(self.$axis.$op_fn(rhs),)+)
			}
		}

		impl<const N: i64> $op<$name> for Fx32Var<N> {
			type Output = $name;

			#[inline]
			fn $op_fn(self, rhs: $name) -> $name {
				<$name>::new($(self.$op_fn(rhs.$axis),)+)
			}
		}
	};
}

macro_rules! impl_op_num {
	($name: ty, $op: ident, $op_fn: ident, $num: ty, $($axis: ident),+) => {
		impl<const N: i64> $op<$num> for $name {
			type Output = $name;

			#[inline]
			fn $op_fn(self, rhs: $num) -> $name {
				<$name>::new($(self.$axis.$op_fn(Into::<Fx32Var<N>>::into(rhs)),)+)
			}
		}

		impl<const N: i64> $op<$name> for $num {
			type Output = $name;

			#[inline]
			fn $op_fn(self, rhs: $name) -> $name {
				<$name>::new($(Into::<Fx32Var<N>>::into(self).$op_fn(rhs.$axis),)+)
			}
		}
	};
}

macro_rules! impl_ops {
	($name: ty, $($axis: ident),+) => {
		impl_op!($name, Add, add, $($axis),+);
		impl_ref!($name, Fx32Var<N>, $name, Add, add);
		impl_ref!(Fx32Var<N>, $name, $name, Add, add);
		impl_assign!($name, Fx32Var<N>, add, AddAssign, add_assign);

		impl_op!($name, Sub, sub, $($axis),+);
		impl_ref!($name, Fx32Var<N>, $name, Sub, sub);
		impl_ref!(Fx32Var<N>, $name, $name, Sub, sub);
		impl_assign!($name, Fx32Var<N>, sub, SubAssign, sub_assign);

		impl_op!($name, Mul, mul, $($axis),+);
		impl_ref!($name, Fx32Var<N>, $name, Mul, mul);
		impl_ref!(Fx32Var<N>, $name, $name, Mul, mul);
		impl_assign!($name, Fx32Var<N>, mul, MulAssign, mul_assign);

		impl_op!($name, Div, div, $($axis),+);
		impl_ref!($name, Fx32Var<N>, $name, Div, div);
		impl_ref!(Fx32Var<N>, $name, $name, Div, div);
		impl_assign!($name, Fx32Var<N>, div, DivAssign, div_assign);
	};
}

macro_rules! impl_ops_num {
	($name: ty, $num: ty, $($axis: ident),+) => {
		impl_op_num!($name, Add, add, $num, $($axis),+);
		impl_ref!($name, $num, $name, Add, add);
		impl_ref!($num, $name, $name, Add, add);
		impl_assign!($name, $num, add, AddAssign, add_assign);

		impl_op_num!($name, Sub, sub, $num, $($axis),+);
		impl_ref!($name, $num, $name, Sub, sub);
		impl_ref!($num, $name, $name, Sub, sub);
		impl_assign!($name, $num, sub, SubAssign, sub_assign);

		impl_op_num!($name, Mul, mul, $num, $($axis),+);
		impl_ref!($name, $num, $name, Mul, mul);
		impl_ref!($num, $name, $name, Mul, mul);
		impl_assign!($name, $num, mul, MulAssign, mul_assign);

		impl_op_num!($name, Div, div, $num, $($axis),+);
		impl_ref!($name, $num, $name, Div, div);
		impl_ref!($num, $name, $name, Div, div);
		impl_assign!($name, $num, div, DivAssign, div_assign);
	};
}

impl_ops!(FxVecVar2<N>, x, y);
impl_ops!(FxVecVar3<N>, x, y, z);
impl_ops!(FxVecVar4<N>, x, y, z, w);

impl_ops_num!(FxVecVar2<N>, f32, x, y);
impl_ops_num!(FxVecVar3<N>, f32, x, y, z);
impl_ops_num!(FxVecVar4<N>, f32, x, y, z, w);

// Arrays and tuples, can't really do these with macros

impl<const N: i64> From<[Fx32Var<N>; 2]> for FxVecVar2<N> {
	fn from(value: [Fx32Var<N>; 2]) -> Self {
		Self::new(value[0], value[1])
	}
}

impl<const N: i64> From<FxVecVar2<N>> for [Fx32Var<N>; 2] {
	fn from(value: FxVecVar2<N>) -> Self {
		[value.x, value.y]
	}
}

impl<const N: i64> From<(Fx32Var<N>, Fx32Var<N>)> for FxVecVar2<N> {
	fn from(value: (Fx32Var<N>, Fx32Var<N>)) -> Self {
		Self::new(value.0, value.1)
	}
}

impl<const N: i64> From<FxVecVar2<N>> for (Fx32Var<N>, Fx32Var<N>) {
	fn from(value: FxVecVar2<N>) -> Self {
		(value.x, value.y)
	}
}

impl<const N: i64> From<[Fx32Var<N>; 3]> for FxVecVar3<N> {
	fn from(value: [Fx32Var<N>; 3]) -> Self {
		Self::new(value[0], value[1], value[2])
	}
}

impl<const N: i64> From<FxVecVar3<N>> for [Fx32Var<N>; 3] {
	fn from(value: FxVecVar3<N>) -> Self {
		[value.x, value.y, value.z]
	}
}

impl<const N: i64> From<(Fx32Var<N>, Fx32Var<N>, Fx32Var<N>)> for FxVecVar3<N> {
	fn from(value: (Fx32Var<N>, Fx32Var<N>, Fx32Var<N>)) -> Self {
		Self::new(value.0, value.1, value.2)
	}
}

impl<const N: i64> From<FxVecVar3<N>> for (Fx32Var<N>, Fx32Var<N>, Fx32Var<N>) {
	fn from(value: FxVecVar3<N>) -> Self {
		(value.x, value.y, value.z)
	}
}

impl<const N: i64> From<[Fx32Var<N>; 4]> for FxVecVar4<N> {
	fn from(value: [Fx32Var<N>; 4]) -> Self {
		Self::new(value[0], value[1], value[2], value[3])
	}
}

impl<const N: i64> From<FxVecVar4<N>> for [Fx32Var<N>; 4] {
	fn from(value: FxVecVar4<N>) -> Self {
		[value.x, value.y, value.z, value.w]
	}
}

impl<const N: i64> From<(Fx32Var<N>, Fx32Var<N>, Fx32Var<N>, Fx32Var<N>)> for FxVecVar4<N> {
	fn from(value: (Fx32Var<N>, Fx32Var<N>, Fx32Var<N>, Fx32Var<N>)) -> Self {
		Self::new(value.0, value.1, value.2, value.3)
	}
}

impl<const N: i64> From<FxVecVar4<N>> for (Fx32Var<N>, Fx32Var<N>, Fx32Var<N>, Fx32Var<N>) {
	fn from(value: FxVecVar4<N>) -> Self {
		(value.x, value.y, value.z, value.w)
	}
}

#[cfg(any(feature = "glam", feature = "bevy_math"))]
macro_rules! impl_conversion {
	($glam: ty, $name: ty, $($axis: ident),+) => {
		impl<const N: i64> From<$name> for $glam {
			fn from(value: $name) -> Self {
				Self::new($(value.$axis.into(),)+)
			}
		}

		impl<const N: i64> From<$glam> for $name {
			fn from(value: $glam) -> Self {
				Self::new($(value.$axis.into(),)+)
			}
		}
	};
}

#[cfg(feature = "glam")]
mod glam {
	use glam::{IVec2, IVec3, IVec4, Vec2, Vec3, Vec4};

	use crate::{FxVecVar2, FxVecVar3, FxVecVar4};

	impl_conversion!(Vec2, FxVecVar2<N>, x, y);
	impl_conversion!(Vec3, FxVecVar3<N>, x, y, z);
	impl_conversion!(Vec4, FxVecVar4<N>, x, y, z, w);

	impl_conversion!(IVec2, FxVecVar2<N>, x, y);
	impl_conversion!(IVec3, FxVecVar3<N>, x, y, z);
	impl_conversion!(IVec4, FxVecVar4<N>, x, y, z, w);
}

#[cfg(feature = "bevy_math")]
mod bevy_math {
	use bevy_math::{IVec2, IVec3, IVec4, Vec2, Vec3, Vec4};

	use crate::{FxVecVar2, FxVecVar3, FxVecVar4};

	impl_conversion!(Vec2, FxVecVar2<N>, x, y);
	impl_conversion!(Vec3, FxVecVar3<N>, x, y, z);
	impl_conversion!(Vec4, FxVecVar4<N>, x, y, z, w);

	impl_conversion!(IVec2, FxVecVar2<N>, x, y);
	impl_conversion!(IVec3, FxVecVar3<N>, x, y, z);
	impl_conversion!(IVec4, FxVecVar4<N>, x, y, z, w);
}
