pub const AFFINE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/affine.o"));
pub const BINARY: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/binary.o"));
pub const CAST: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cast.o"));
pub const CONV: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/conv.o"));
pub const FILL: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fill.o"));
pub const INDEXING: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/indexing.o"));
pub const REDUCE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/reduce.o"));
pub const TERNARY: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ternary.o"));
pub const UNARY: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/unary.o"));
