use sfml::{
    cpp::FBox,
    graphics::{IntRect, Texture},
    SfResult,
};
pub use stampcollect::{self, collect};
use stampcollect::{AtlasBuilder, PxRect, PxSc, PxVec};

pub struct SfmlAtlasBuilder {
    atlas_texture: FBox<Texture>,
    current_loaded_texture: Option<FBox<Texture>>,
}

impl SfmlAtlasBuilder {
    pub fn with_size(w: u16, h: u16) -> SfResult<Self> {
        let mut tex = Texture::new().unwrap();
        tex.create(w.into(), h.into())?;
        Ok(Self {
            atlas_texture: tex,
            current_loaded_texture: None,
        })
    }
    pub fn into_texture(self) -> FBox<Texture> {
        self.atlas_texture
    }
}

impl AtlasBuilder for SfmlAtlasBuilder {
    fn load_image(&mut self, path: &std::path::Path) -> PxVec {
        let texture = Texture::from_file(path.to_str().unwrap()).unwrap();
        let size = texture.size();
        self.current_loaded_texture = Some(texture);
        PxVec {
            x: size.x as PxSc,
            y: size.y as PxSc,
        }
    }

    fn integrate(&mut self, pos: PxVec) {
        unsafe {
            self.atlas_texture.update_from_texture(
                self.current_loaded_texture.as_ref().unwrap(),
                pos.x.into(),
                pos.y.into(),
            );
        }
    }

    fn max_size(&self) -> PxVec {
        let size = self.atlas_texture.size();
        PxVec {
            x: size.x as PxSc,
            y: size.y as PxSc,
        }
    }
}

pub trait PxRectExt {
    fn to_sf_intrect(&self) -> IntRect;
}

impl PxRectExt for PxRect {
    fn to_sf_intrect(&self) -> IntRect {
        IntRect {
            left: self.x.into(),
            top: self.y.into(),
            width: self.w.into(),
            height: self.h.into(),
        }
    }
}
