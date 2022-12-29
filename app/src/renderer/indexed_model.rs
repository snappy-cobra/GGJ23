use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use ogc_rs::{print, println};
use wavefront::{Index, Obj, Vertex};

/**
 * Our representation of a model.
 */
pub struct IndexedModel {
    pub positions: Vec<f32>,
    pub position_indices: Vec<u16>,
    pub tex_coords: Vec<f32>,
    pub tex_coord_indices: Vec<u16>
}

pub const BYTE_SIZE_F32: u8 = 4;
pub const SIZE_POSITION: u8 = 3;
pub const BYTE_SIZE_POSITION: u8 = BYTE_SIZE_F32 * SIZE_POSITION;
pub const SIZE_TEX_COORD: u8 = 2;
pub const BYTE_SIZE_TEX_COORD: u8 = BYTE_SIZE_F32 * SIZE_TEX_COORD;

/**
 * Implementation of the indexed model.
 */
impl IndexedModel {
    /**
     * Turn a model into its indexed equivalent.
     *
     * This is done by filling memotables whose keys are known indexable vertex attributes that we have seen before,
     * and whose values are indexes into an array containing these vertex attributes.
     */
    pub fn new(obj_data: &Obj) -> IndexedModel {
        let mut vertex_memo: BTreeMap<Index, u16> = BTreeMap::new();
        let mut tex_coord_memo: BTreeMap<Index, u16> = BTreeMap::new();
        let mut positions = Vec::new();
        let mut tex_coords: Vec<f32> = Vec::new();
        
        // Add vertex positions
        let position_indices = obj_data
            .vertices()
            .map(|vertex| {
                let vertex_id = vertex.position_index();
                *vertex_memo.entry(vertex_id).or_insert_with(|| {
                    let index = (positions.len() / SIZE_POSITION as usize) as u16;
                    positions.extend(vertex.position());
                    index
                })
            })
            .collect();

        // Add tex coords
        let tex_coord_indices = obj_data
            .vertices()
            .map(|vertex| {
                let tex_coord_id = vertex.uv_index().unwrap_or(0usize);
                *tex_coord_memo.entry(tex_coord_id).or_insert_with(|| {
                    let index = (tex_coords.len() / SIZE_TEX_COORD as usize) as u16;
                    let uvw = vertex.uv().unwrap_or([0.0, 0.0, 0.0]);
                    tex_coords.push(uvw[0]);
                    tex_coords.push(1.0 - uvw[1]);
                    index
                })
            })
            .collect();

        IndexedModel {
            positions,
            position_indices,
            tex_coords,
            tex_coord_indices,
        }
    }
}
