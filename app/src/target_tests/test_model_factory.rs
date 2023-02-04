use crate::rendering::model_factory::ModelFactory;
use gamelib::data_store::asset_name::AssetName;
use gamelib::data_store::textured_model_name::TexturedModelName;
use hashbrown::HashMap;

/**
 * Test if the model factory behaves as expected.
 */
pub fn test_model_factory(tests: &mut HashMap<&'static str, fn()>) {
    tests.insert("Loading a textured cube.", test_textured_cube);
}

/**
 * Test if we load a valid textured cube.
 */
fn test_textured_cube() {
    let mut factory = ModelFactory::new();
    factory.load_models();
    let textured_model = factory.get_model(&TexturedModelName::Cube).unwrap();

    // Check if the attributes are non-empty and of the right size
    assert!(!textured_model.model.positions.is_empty());
    assert!(textured_model.model.positions.len() % 3 == 0);
    assert!(!textured_model.model.tex_coords.is_empty());
    assert!(textured_model.model.tex_coords.len() % 2 == 0);

    // Check if the indexes are non-empty and of the same size
    assert!(!textured_model.model.position_indices.is_empty());
    assert!(!textured_model.model.tex_coord_indices.is_empty());
    assert!(
        textured_model.model.position_indices.len() == textured_model.model.tex_coord_indices.len()
    );
}
