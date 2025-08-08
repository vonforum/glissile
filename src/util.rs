macro_rules! impl_ref {
	($lhs: ty, $rhs: ty, $output: ty, $op: ident, $op_fn: ident) => {
		impl<const N: i64> $op<&$rhs> for $lhs {
			type Output = $output;

			#[inline]
			fn $op_fn(self, rhs: &$rhs) -> $output {
				self.$op_fn(*rhs)
			}
		}

		impl<const N: i64> $op<$rhs> for &$lhs {
			type Output = $output;

			#[inline]
			fn $op_fn(self, rhs: $rhs) -> $output {
				(*self).$op_fn(rhs)
			}
		}

		impl<const N: i64> $op<&$rhs> for &$lhs {
			type Output = $output;

			#[inline]
			fn $op_fn(self, rhs: &$rhs) -> $output {
				(*self).$op_fn(*rhs)
			}
		}
	};

	($name: ty, $op: ident, $op_fn: ident) => {
		impl_ref!($name, $name, $name, $op, $op_fn);
	};
}

macro_rules! impl_assign {
	($lhs: ty, $rhs: ty, $op_fn: ident, $op_assign: ident, $op_assign_fn: ident) => {
		impl<const N: i64> $op_assign<$rhs> for $lhs {
			#[inline]
			fn $op_assign_fn(&mut self, other: $rhs) {
				*self = self.$op_fn(other);
			}
		}

		impl<const N: i64> $op_assign<&$rhs> for $lhs {
			#[inline]
			fn $op_assign_fn(&mut self, other: &$rhs) {
				*self = self.$op_fn(*other);
			}
		}
	};
}

pub(super) use impl_assign;
pub(super) use impl_ref;
