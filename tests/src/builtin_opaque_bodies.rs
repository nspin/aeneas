//@ [!lean] skip
//@ charon-args=--extract-opaque-bodies
//! Regression test: a function matched to a builtin (here `core::mem::swap`)
//! must be extracted with the builtin's effects even when Charon provides its
//! body. Without this, the body (a call to the opaque
//! `typed_swap_nonoverlapping` intrinsic) made the analysis consider that
//! `swap` can fail, while the builtin `core.mem.swap` is pure, leading to a
//! type mismatch in the generated code.

fn swap<T>(x: &mut T, y: &mut T) {
    core::mem::swap(x, y)
}
