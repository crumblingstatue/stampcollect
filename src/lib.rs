//! Collects images from a directory tree into a texture atlas, using their
//! relative path as string ids.

#![warn(missing_docs)]

use std::path::Path;

use etagere::{AtlasAllocator, Size};
use walkdir::WalkDir;

/// Atlas pixel position scalar
pub type PxSc = u16;

/// Atlas pixel position 2d vector
pub struct PxVec {
    /// Horizontal position
    pub x: PxSc,
    /// Vertical position
    pub y: PxSc,
}

/// Atlas pixel position rectangle
pub struct PxRect {
    /// Horizontal position
    pub x: PxSc,
    /// Vertical position
    pub y: PxSc,
    /// Width
    pub w: PxSc,
    /// Height
    pub h: PxSc,
}

/// A type that can load images and build a texture atlas by following packing instructions
pub trait AtlasBuilder {
    /// Load an image and return the size of the loaded image
    fn load_image(&mut self, path: &Path) -> PxVec;
    /// Copy the currently loaded image into the atlas at `pos`
    fn integrate(&mut self, pos: PxVec);
    /// Report the maximum size of this atlas to pack the images into
    fn max_size(&self) -> PxVec;
}

/// Collect images from `dir` and build a texture atlas out of them.
///
/// `meta_cb` is a callback with the name and rectangle of every packed image, to build
/// your own atlas metadata from.
pub fn collect<P: AsRef<Path>, A: AtlasBuilder, C: FnMut(String, PxRect)>(
    dir: P,
    atlas: &mut A,
    mut meta_cb: C,
) {
    let max_size = atlas.max_size();
    let mut alloc = AtlasAllocator::new(Size::new(max_size.x.into(), max_size.y.into()));
    for entry in WalkDir::new(dir.as_ref()).into_iter() {
        let en = entry.unwrap();
        if en.metadata().unwrap().is_file() {
            let path = en.path();
            let img_size = atlas.load_image(path);
            let rect = alloc
                .allocate(Size::new(img_size.x.into(), img_size.y.into()))
                .unwrap()
                .rectangle;
            atlas.integrate(PxVec {
                x: rect.min.x as u16,
                y: rect.min.y as u16,
            });
            let size = rect.size();
            meta_cb(
                path_to_id(path, dir.as_ref()),
                PxRect {
                    x: rect.min.x as u16,
                    y: rect.min.y as u16,
                    w: size.width as u16,
                    h: size.height as u16,
                },
            );
        }
    }
}

fn path_to_id(path: &Path, base: &Path) -> String {
    let stripped = path.strip_prefix(base).unwrap();
    let mut s = stripped.to_string_lossy().into_owned();
    if let Some(ext_pos) = s.rfind('.') {
        s.truncate(ext_pos)
    }
    s
}

#[test]
fn test_path_to_id() {
    assert_eq!(
        &path_to_id("home/foo/bar/baz.png".as_ref(), "home/".as_ref()),
        "foo/bar/baz"
    );
}
