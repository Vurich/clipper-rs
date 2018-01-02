#![cfg_attr(test, feature(test))]

mod bindings;

pub use bindings::root::ClipperLib as clipper_lib;
pub use bindings::root::{clipper, path, paths};

pub use bindings::root::{path_clear, path_free, path_new, path_size, path_addPoint, path_getArea,
                         path_getPointX, path_getPointY};

pub use bindings::root::{paths_clear, paths_free, paths_new, paths_size, paths_addPath,
                         paths_getPath};

pub use bindings::root::{clipper_execute, clipper_free, clipper_new, clipper_addPath,
                         clipper_addPaths};

pub use bindings::root::{clipperoffset_execute, clipperoffset_free, clipperoffset_new, clipperoffset_setArcTolerance, clipperoffset_setMiterLimit,
                         clipperoffset_addPath, clipperoffset_addPaths};

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::black_box;

    #[test]
    fn everything_linked() {
        use std::os::raw::c_void;

        let not_true = black_box(false);

        if not_true {
            black_box(
                [
                    path_new as *const c_void,
                    path_clear as _,
                    path_size as _,
                    path_addPoint as _,
                    path_getPointX as _,
                    path_getPointY as _,
                    path_free as _,
                    path_getArea as _,
                    paths_new as _,
                    paths_clear as _,
                    paths_size as _,
                    paths_getPath as _,
                    paths_addPath as _,
                    paths_free as _,
                    clipper_new as _,
                    clipper_addPath as _,
                    clipper_addPaths as _,
                    clipper_execute as _,
                    clipper_free as _,
                    clipperoffset_new as _,
                    clipperoffset_setMiterLimit as _,
                    clipperoffset_setArcTolerance as _,
                    clipperoffset_addPath as _,
                    clipperoffset_addPaths as _,
                    clipperoffset_execute as _,
                    clipperoffset_free as _,
                ]
            );
        }
    }
}
