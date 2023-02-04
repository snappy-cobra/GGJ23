use alloc::collections::BTreeMap;
use alloc::str::from_utf8;
use alloc::vec::Vec;
use ogc_rs::print;
use strum::IntoEnumIterator;
use wavefront::Obj;

use gamelib::data_store::asset_name::AssetName;

use grrustlib::GRRLIB_LoadTTF;
use grrustlib::GRRLIB_ttfFont;

/**
 * Data structure for the font factory.
 */
#[derive(Debug)]
pub struct TextFactory {
    pub font: Option<*mut GRRLIB_ttfFont>,
}

/**
 * Implementation of the font factory: allows for preloading and fetching font data.
 */
impl TextFactory {
    /**
     * Create a new factory.
     */
    pub fn new() -> TextFactory {
        let mut res: Self = TextFactory { font: None };
        res.load_fonts();
        res
    }

    /**
     * Load all fonts.
     */
    pub fn load_fonts(&mut self) {
        let font_data = AssetName::FreeMonoBold.to_data();
        unsafe {
            let temp = GRRLIB_LoadTTF(&font_data[0], font_data.len() as i32);
            self.font = Some(temp);
        }
    }
}
