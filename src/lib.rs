extern crate clipper_sys as sys;

use std::mem::ManuallyDrop;
use std::iter::{self, FromIterator};
use std::ops::Deref;
use std::marker::PhantomData;

macro_rules! debug {
    () => {
        println!("{}:{},{}", file!(), line!(), column!());
    }
}

// This is just Vec<(u64, u64)>, but C++ and Rust don't share an allocator
// Needs to be `repr(C)` so we can construct references to it from the `*void` pointer returned
// from C
#[repr(C)]
pub struct Path {
    internal: sys::path,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PathRef<'a> {
    internal: sys::path,
    _marker: PhantomData<&'a Path>,
}

impl<'a> PathRef<'a> {
    pub fn len(self) -> usize {
        // TODO: Are we vulnerable to underflow?
        (unsafe { sys::path_size(self.internal) }) as usize
    }

    pub fn get(self, index: usize) -> Option<(i64, i64)> {
        if index >= self.len() {
            None
        } else {
            unsafe {
                Some((
                    sys::path_getPointX(self.internal, index as i32),
                    sys::path_getPointY(self.internal, index as i32),
                ))
            }
        }
    }

    pub fn iter(self) -> PathIter<'a> {
        PathIter {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> Deref for PathRef<'a> {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl Path {
    pub fn len(&self) -> usize {
        // TODO: Are we vulnerable to underflow?
        (unsafe { sys::path_size(self.internal) }) as usize
    }

    pub fn get(&self, index: usize) -> Option<(i64, i64)> {
        if index >= self.len() {
            None
        } else {
            unsafe {
                Some((
                    sys::path_getPointX(self.internal, index as i32),
                    sys::path_getPointY(self.internal, index as i32),
                ))
            }
        }
    }

    pub fn iter(&self) -> PathIter {
        PathIter {
            inner: PathRef {
                internal: self.internal,
                _marker: PhantomData,
            },
            index: 0,
        }
    }
}

// TODO: Convert this to get the array start and end pointers from C++
pub struct PathIter<'a> {
    inner: PathRef<'a>,
    index: usize,
}

impl<'a> Iterator for PathIter<'a> {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(out) = self.inner.get(self.index) {
            self.index += 1;
            Some(out)
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;

        let mut iter = self.iter();

        if let Some(point) = iter.next() {
            write!(f, "{:?}", point)?;
        }

        for point in iter {
            write!(f, ", {:?}", point)?;
        }

        write!(f, "]")
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut index = 0;
        while let (Some(a), Some(b)) = (self.get(index), other.get(index)) {
            if a != b {
                return false;
            }
            index += 1;
        }

        true
    }
}

impl FromIterator<(i64, i64)> for Path {
    fn from_iter<I: IntoIterator<Item = (i64, i64)>>(verts: I) -> Self {
        let verts = verts.into_iter();

        let internal = unsafe { sys::path_new(0) };

        for (x, y) in verts {
            unsafe { sys::path_addPoint(internal, x, y) };
        }

        Path { internal }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { sys::path_free(self.internal) };
    }
}

// This is just Vec<Path>, but C++ and Rust don't share an allocator
// TODO: Use a `(cap, len, ptr)` representation so we don't have to allocate a `void*`
pub struct Paths {
    internal: sys::paths,
}

impl Paths {
    pub fn iter(&self) -> PathsIter {
        PathsIter {
            inner: self,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        // TODO: Are we vulnerable to underflow?
        (unsafe { sys::paths_size(self.internal) }) as usize
    }

    pub fn get(&self, index: usize) -> Option<PathRef> {
        use std::mem;

        if index >= self.len() {
            None
        } else {
            unsafe {
                Some(PathRef {
                    internal: sys::paths_getPath(self.internal, index as i32),
                    _marker: PhantomData,
                })
            }
        }
    }
}

impl FromIterator<Path> for Paths {
    fn from_iter<I: IntoIterator<Item = Path>>(paths: I) -> Self {
        use std::mem;

        let internal = unsafe { sys::paths_new(0) };

        for path in paths {
            unsafe { sys::paths_addPath(internal, path.internal) };
            mem::forget(path);
        }

        Paths { internal }
    }
}

// TODO: Convert this to get the array start and end pointers from C++
pub struct PathsIter<'a> {
    inner: &'a Paths,
    index: usize,
}

impl<'a> Iterator for PathsIter<'a> {
    type Item = PathRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(out) = self.inner.get(self.index) {
            self.index += 1;
            Some(out)
        } else {
            None
        }
    }
}

impl Drop for Paths {
    fn drop(&mut self) {
        unsafe { sys::paths_free(self.internal) };
    }
}

pub enum JoinType {
    Miter { limit: f64 },
    Round { tolerance: f64 },
    Square,
}

impl JoinType {
    fn to_sys_jointype(&self) -> sys::clipper_lib::JoinType {
        match *self {
            JoinType::Miter { .. } => sys::clipper_lib::JoinType_jtMiter,
            JoinType::Round { .. } => sys::clipper_lib::JoinType_jtRound,
            JoinType::Square => sys::clipper_lib::JoinType_jtSquare,
        }
    }
}

pub fn offset<'a, I: IntoIterator<Item = &'a Path>>(
    paths: I,
    join_type: JoinType,
    delta: f64,
) -> Paths {
    let out = unsafe {
        // TODO: Do this in C so we don't have to box `Clipper` just to do this (will this be elided
        //       by LLVM if we use LTO?)
        let clipper = sys::clipperoffset_new();

        match join_type {
            JoinType::Miter { limit } => sys::clipperoffset_setMiterLimit(clipper, limit),
            JoinType::Round { tolerance } => sys::clipperoffset_setArcTolerance(clipper, tolerance),
            _ => {}
        }

        let sys_jointype = join_type.to_sys_jointype();

        for i in paths {
            sys::clipperoffset_addPath(
                clipper,
                i.internal,
                sys_jointype,
                sys::clipper_lib::EndType_etClosedPolygon,
            );
        }

        // TODO: Can we preallocate the minimum size without overallocating?
        let out = sys::paths_new(0);
        sys::clipperoffset_execute(clipper, out, delta);

        sys::clipperoffset_free(clipper);

        out
    };

    Paths { internal: out }
}

#[repr(u32)]
pub enum Operation {
    Intersection = sys::clipper_lib::ClipType_ctIntersection,
    Union = sys::clipper_lib::ClipType_ctUnion,
    Difference = sys::clipper_lib::ClipType_ctDifference,
    Xor = sys::clipper_lib::ClipType_ctXor,
}

pub fn execute<'a, Subject, Clip>(subject: Subject, clip: Clip, operation: Operation) -> Paths
where
    Subject: IntoIterator<Item = &'a Path>,
    Clip: IntoIterator<Item = &'a Path>,
{
    #[repr(u32)]
    enum PolyType {
        Subject = sys::clipper_lib::PolyType_ptSubject,
        Clip = sys::clipper_lib::PolyType_ptClip,
    }

    let out = unsafe {
        // TODO: Do this in C so we don't have to box `Clipper` just to do this (will this be elided
        //       by LLVM if we use LTO?)
        let clipper = sys::clipper_new();

        for i in subject {
            sys::clipper_addPath(clipper, i.internal, PolyType::Subject as _);
        }

        for i in clip {
            sys::clipper_addPath(clipper, i.internal, PolyType::Clip as _);
        }

        // TODO: Can we preallocate the minimum size without overallocating?
        let out = sys::paths_new(0);
        sys::clipper_execute(clipper, operation as _, out);

        sys::clipper_free(clipper);

        out
    };

    Paths { internal: out }
}

pub fn intersection<'a, Subject, Clip>(subject: Subject, clip: Clip) -> Paths
where
    Subject: IntoIterator<Item = &'a Path>,
    Clip: IntoIterator<Item = &'a Path>,
{
    execute(subject, clip, Operation::Intersection)
}

pub fn union<'a, Subject, Clip>(subject: Subject, clip: Clip) -> Paths
where
    Subject: IntoIterator<Item = &'a Path>,
    Clip: IntoIterator<Item = &'a Path>,
{
    execute(subject, clip, Operation::Union)
}

pub fn difference<'a, Subject, Clip>(subject: Subject, clip: Clip) -> Paths
where
    Subject: IntoIterator<Item = &'a Path>,
    Clip: IntoIterator<Item = &'a Path>,
{
    execute(subject, clip, Operation::Difference)
}

pub fn xor<'a, Subject, Clip>(subject: Subject, clip: Clip) -> Paths
where
    Subject: IntoIterator<Item = &'a Path>,
    Clip: IntoIterator<Item = &'a Path>,
{
    execute(subject, clip, Operation::Xor)
}

pub trait PathsExt<'a>: Sized {
    type IntoIter: IntoIterator<Item = &'a Path>;

    fn into_path_iter(self) -> Self::IntoIter;

    fn execute<Clip: PathsExt<'a>>(self, clip: Clip, op: Operation) -> Paths {
        execute(self.into_path_iter(), clip.into_path_iter(), op)
    }

    fn offset(self, join_type: JoinType, delta: f64) -> Paths {
        offset(self.into_path_iter(), join_type, delta)
    }

    fn intersection<Clip: PathsExt<'a>>(self, clip: Clip) -> Paths {
        self.execute(clip, Operation::Intersection)
    }

    fn union<Clip: PathsExt<'a>>(self, clip: Clip) -> Paths {
        self.execute(clip, Operation::Union)
    }

    fn difference<Clip: PathsExt<'a>>(self, clip: Clip) -> Paths {
        self.execute(clip, Operation::Difference)
    }

    fn xor<Clip: PathsExt<'a>>(self, clip: Clip) -> Paths {
        self.execute(clip, Operation::Xor)
    }
}

impl<'a, T> PathsExt<'a> for T
where
    T: IntoIterator<Item = &'a Path>,
{
    type IntoIter = Self;

    fn into_path_iter(self) -> Self::IntoIter {
        self
    }
}

impl<'a> PathsExt<'a> for &'a Path {
    type IntoIter = iter::Once<Self>;

    fn into_path_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset() {
        use std::collections::HashSet;

        let path = [(0, 0), (0, 1), (1, 1), (1, 0)]
            .iter()
            .cloned()
            .collect::<Path>();

        let offset = path.offset(JoinType::Square, 1.);

        assert_eq!(
            offset
                .iter()
                .flat_map(|path| path.iter())
                .collect::<HashSet<_>>(),
            [
                (2, 0),
                (2, 1),
                (1, 2),
                (0, 2),
                (-1, 1),
                (-1, 0),
                (0, -1),
                (1, -1),
            ].iter()
                .cloned()
                .collect::<HashSet<_>>()
        )
    }

    #[test]
    fn union() {
        use std::collections::HashSet;

        let path_a = [(0, 0), (0, 1), (1, 1), (1, 0)]
            .iter()
            .cloned()
            .collect::<Path>();

        let path_b = [(0, 1), (0, 2), (1, 2), (1, 1)]
            .iter()
            .cloned()
            .collect::<Path>();

        let result = path_a.union(&path_b);

        assert_eq!(
            result
                .iter()
                .flat_map(|path| path.iter())
                .collect::<HashSet<_>>(),
            [(0, 0), (0, 2), (1, 2), (1, 0),]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        )
    }

    #[test]
    fn difference() {
        use std::collections::HashSet;

        let path_a = [(0, 0), (0, 2), (1, 2), (1, 0)]
            .iter()
            .cloned()
            .collect::<Path>();

        let path_b = [(0, 1), (0, 2), (1, 2), (1, 1)]
            .iter()
            .cloned()
            .collect::<Path>();

        let result = path_a.difference(&path_b);

        assert_eq!(
            result
                .iter()
                .flat_map(|path| path.iter())
                .collect::<HashSet<_>>(),
            [(0, 0), (0, 1), (1, 1), (1, 0)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        )
    }

    #[test]
    fn xor() {
        use std::collections::HashSet;

        let path_a = [(0, 0), (0, 2), (1, 2), (1, 0)]
            .iter()
            .cloned()
            .collect::<Path>();

        let path_b = [(0, 1), (0, 2), (2, 2), (2, 1)]
            .iter()
            .cloned()
            .collect::<Path>();

        let result = path_a.xor(&path_b);

        assert_eq!(
            result
                .iter()
                .flat_map(|path| path.iter())
                .collect::<HashSet<_>>(),
            [
                (0, 0),
                (0, 1),
                (1, 1),
                (1, 0),

                (1, 1),
                (1, 2),
                (2, 2),
                (2, 1)
            ].iter()
                .cloned()
                .collect::<HashSet<_>>()
        )
    }

    #[test]
    fn intersection() {
        use std::collections::HashSet;

        let path_a = [(0, 0), (0, 2), (1, 2), (1, 0)]
            .iter()
            .cloned()
            .collect::<Path>();

        let path_b = [(0, 1), (0, 2), (2, 2), (2, 1)]
            .iter()
            .cloned()
            .collect::<Path>();

        let result = path_a.intersection(&path_b);

        assert_eq!(
            result
                .iter()
                .flat_map(|path| path.iter())
                .collect::<HashSet<_>>(),
            [(0, 1), (0, 2), (1, 2), (1, 1)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        )
    }
}
