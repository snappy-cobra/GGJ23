use super::display_cache::DisplayCache;
use super::indexed_model::{BYTE_SIZE_POSITION, BYTE_SIZE_TEX_COORD};
use super::model_factory::ModelFactory;
use super::text_factory::TextFactory;
use super::textured_model::TexturedModel;
use gamelib::data_store::asset_name::AssetName;
use gamelib::data_store::textured_model_name::TexturedModelName;
use gamelib::game_state::components::render::MeshInstance;
use gamelib::{
    game_state::components::motion::Position, game_state::components::motion::Velocity,
    game_state::components::render::Text, game_state::GameState, servers::renderer::RenderServer,
};
use grrustlib::*;
use hecs::*;
use libc::c_void;
use ogc_rs::prelude::Vec;
use ogc_rs::{print, println};
use wavefront::{Obj, Vertex};

/// Representation of the graphics rendering subsystem of the device
///
/// As the device only has _one_ graphics chip which is exposed as a globally mutable state machine,
/// at most one Renderer should be constructed at any time.
///
/// Graphics setup happens as part of initialization,
/// and cleanup happens automatically on drop.
pub struct WiiRenderServer {
    model_factory: ModelFactory,
    text_factory: TextFactory,
    display_cache: DisplayCache,
}

impl WiiRenderServer {
    ///
    /// Create a new renderer.
    ///
    /// As part of this:
    /// - the graphics chip is initialized in the expected rendering mode.
    /// - The available models are constructed and indexed. (c.f. `ModelFactory`)
    pub fn new() -> Self {
        let res = Self {
            model_factory: ModelFactory::new(),
            text_factory: TextFactory::new(),
            display_cache: DisplayCache::new(),
        };
        res.init_render();
        res
    }

    /**
     * Initialize the renderer, which means GRRLIB and loading all models.
     */
    fn init_render(&self) {
        unsafe {
            GRRLIB_Init();
            GRRLIB_Settings.antialias = true;

            GRRLIB_SetBackgroundColour(0x00, 0x00, 0x00, 0xFF);
            GRRLIB_Camera3dSettings(0.0, 0.0, 13.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        }
    }

    /// Render a single entity
    fn render_entity(&mut self, model_name: &TexturedModelName, position: &Position) {
        let temp = self.text_factory.font.unwrap();

        unsafe {
            GRRLIB_PrintfTTF(50, 50, temp, "TEST".as_ptr(), 12, 0xFFFFFFFF);

            GRRLIB_3dMode(0.1, 1000.0, 45.0, false, false);
            GRRLIB_ObjectView(
                position.x, position.y, position.z, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
            );
            self.render_textured_model(model_name);
        }
    }

    fn render_text(&mut self, text: &Text) {
        // Fill a table with characters
        // let i: libc::wchar_t = 0;
        // let mut n: usize = 0;
        // let mut charTable: [libc::wchar_t; 460] = [0; 460];
        // for i in 33..126 {
        //     charTable[n] = i;
        //     n += 1;
        // }
        // for i in 161..518 {
        //     charTable[n] = i;
        //     n += 1;
        // }
        // for i in 9824..9831 {
        //     charTable[n] = i;
        //     n += 1;
        // }

        // let mut Letter: [libc::wchar_t; 10] = [0; 10];
        // Letter[0] = charTable[12];
        // Letter[1] = charTable[22];
        // Letter[2] = charTable[32];
        // Letter[3] = charTable[42];
        // Letter[4] = charTable[52];
        // Letter[5] = charTable[62];
        // Letter[6] = charTable[72];
        // Letter[7] = charTable[82];

        // let mut FPS[255];
        // snprintf(FPS, sizeof(FPS), "Current FPS: %d", 30.0);

        let temp = self.text_factory.font.unwrap();
        unsafe {
            GRRLIB_PrintfTTF(500, 25, temp, "TEST".as_ptr(), 12, 0xFFFFFFFF);
        }
    }

    /**
     * Renders the given model at whatever position was set previously using other calls into GRRLIB / GX.
     */
    fn render_textured_model(&mut self, model_name: &TexturedModelName) {
        let textured_model = self.model_factory.get_model(model_name).unwrap();
        textured_model.texture.set_active(true);
        Self::pass_textured_model_data(textured_model);
        Self::pass_textured_model_description();

        let display_list = self.display_cache.get_display_list(model_name);
        if !display_list.is_initialized() {
            display_list.open();
            Self::pass_textured_model_data_indices(textured_model);
            display_list.close();
        }
        display_list.set_active();
    }

    /**
     * Describe the data format we push to the GPU as indexed data.
     */
    fn pass_textured_model_description() {
        unsafe {
            GX_SetVtxDesc(GX_VA_POS as u8, GX_INDEX16 as u8);
            GX_SetVtxDesc(GX_VA_CLR0 as u8, GX_DIRECT as u8);
            GX_SetVtxDesc(GX_VA_TEX0 as u8, GX_INDEX16 as u8);
            GX_SetVtxAttrFmt(GX_VTXFMT0 as u8, GX_VA_POS, GX_POS_XYZ, GX_F32, 0);
            GX_SetVtxAttrFmt(GX_VTXFMT0 as u8, GX_VA_TEX0, GX_TEX_ST, GX_F32, 0);
        }
    }

    /**
     * Sets pointers to the textured model data for the GPU to access.
     *
     * ## Safety
     * We call GX_SetArray which takes a pointer into the vertices of the model as '*void *' (C syntax) AKA '*mut c_void' (Rust syntax).
     * By checking the implementation of GX_SetArray it is clear that this signature is wrong; the argument is only used for reading and not mutated.
     * In other words: The argument is treated as if it were a 'const *void' (C syntax) AKA '*const c_void' (Rust syntax).
     * As such, it is OK to turn the immutable reference into a mutable pointer.
     */
    fn pass_textured_model_data(textured_model: &TexturedModel) {
        let positions_ptr = textured_model.model.positions.as_ptr().cast_mut() as *mut c_void;
        let tex_coord_ptr = textured_model.model.tex_coords.as_ptr().cast_mut() as *mut c_void;
        unsafe {
            GX_SetArray(GX_VA_POS, positions_ptr, BYTE_SIZE_POSITION as u8);
            GX_SetArray(GX_VA_TEX0, tex_coord_ptr, BYTE_SIZE_TEX_COORD as u8);
        }
    }

    /**
     * Iterate over the index arrays and set them in direct mode for the GPU to use.
     * Expects data to be described and passed before being called.
     */
    fn pass_textured_model_data_indices(textured_model: &TexturedModel) {
        unsafe {
            // Provide all the indices (wii really wants this in direct mode it seems)
            GX_Begin(
                GX_TRIANGLES as u8,
                GX_VTXFMT0 as u8,
                textured_model.model.position_indices.len() as u16,
            );
            let vertex_count = textured_model.model.position_indices.len();
            let position_indices = textured_model.model.position_indices.to_vec();
            let tex_coord_indices = textured_model.model.tex_coord_indices.to_vec();
            for index in 0..vertex_count {
                GX_Position1x16(position_indices[index]);
                GX_Color1u32(0xFFFFFFFF);
                GX_TexCoord1x16(tex_coord_indices[index]);
            }
            GX_End();
        }
    }
}

impl Drop for WiiRenderServer {
    /// Cleanup the renderer
    fn drop(&mut self) {
        println!("Dropping Renderer");
        unsafe {
            GRRLIB_Exit();
        }
    }
}

/**
 * Implement the render state implementation for the game to use.
 */
impl RenderServer for WiiRenderServer {
    /*
     * Render all given meshes.
     * As part of this, refreshes the graphics buffer and wait for the next frame.
     */
    fn render_meshes(&mut self, meshes: Vec<(&MeshInstance, &Position)>) {
        for (mesh_instance, position) in meshes {
            self.render_entity(&mesh_instance.model_name, position);
        }
    }

    fn render_text(&mut self, texts: Vec<&Text>) {
        unsafe {
            GRRLIB_DrawImg(0.0, 0.0, copiedImg, 0.0, 1.0, 1.0, 0xFFFFFFFF);

            for text in texts {
                self.render_text(&text);
            }

            GRRLIB_Screen2Texture(0, 0, copiedImg, false);
        }
    }

    /**
     * Render a new frame.
     */
    fn render_frame(&mut self) {
        unsafe {
            GRRLIB_Render();
        }
    }
}
