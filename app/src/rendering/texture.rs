use core::ptr::null;

use alloc::vec::Vec;
use grrustlib::{GRRLIB_LoadTexture, GRRLIB_SetTexture, GRRLIB_texImg};

/**
 * Our representation of a texture.
 */
#[derive(Debug)]
pub struct Texture {
    grrlib_texture: *mut GRRLIB_texImg,
}

/**
 * Implementation of texture.
 */
impl Texture {
    /**
     * Load a new texture based on PNG image data.
     */
    pub fn from_bytes(png_data: impl AsRef<[u8]>) -> Result<Texture, &'static str> {
        unsafe {
            let grrlib_texture = GRRLIB_LoadTexture(png_data.as_ref().as_ptr());
            if grrlib_texture.is_null() {
                return Err("Image could not be loaded");
            }
            Ok(Texture { grrlib_texture })
        }
    }

    /**
     * Set this texture to the active texture
     */
    pub fn set_active(&self, is_repeating: bool) {
        unsafe {
            GRRLIB_SetTexture(self.grrlib_texture, is_repeating);
        }
    }
}
