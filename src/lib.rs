pub mod num;
mod util;
pub mod vec;

pub use num::*;
pub use vec::*;

pub const DEFAULT_RESOLUTION: i64 = u16::MAX as i64 + 1;

pub type Fx32 = Fx32Var<DEFAULT_RESOLUTION>;
pub type FxVec2 = FxVecVar2<DEFAULT_RESOLUTION>;
pub type FxVec3 = FxVecVar3<DEFAULT_RESOLUTION>;
pub type FxVec4 = FxVecVar4<DEFAULT_RESOLUTION>;
