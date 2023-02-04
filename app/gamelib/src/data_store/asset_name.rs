/**
 * Enumerates all models that exist in the project.
 * Each of them can be turned into its actual raw data by calling `to_data()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum AssetName {
    Cube,
    Suzanne,
    Triangle,
    CubeTexture,
    TriangleTexture,
    SuzanneTexture,
    DemoMusic,
    BoingSFX,
    FreeMonoBold,
}

impl AssetName {
    /// Returns the raw model data of this particular ModelName
    ///
    /// Internally, `include_bytes!` is used
    /// so each of the files in the ../data directory is included at compile time.
    pub const fn to_data(&self) -> &'static [u8] {
        match self {
            AssetName::Cube => include_bytes!("data/Cube.obj"),
            AssetName::Suzanne => include_bytes!("data/Suz.obj"),
            AssetName::Triangle => include_bytes!("data/Tri.obj"),
            AssetName::CubeTexture => include_bytes!("data/Cube.png"),
            AssetName::TriangleTexture => include_bytes!("data/Tri.png"),
            AssetName::SuzanneTexture => include_bytes!("data/Suz.png"),
            AssetName::DemoMusic => include_bytes!("data/DemoMusic.ogg"),
            AssetName::BoingSFX => include_bytes!("data/Boing.ogg"),
            AssetName::FreeMonoBold => include_bytes!("data/FreeMonoBold.ttf"),
        }
    }
}
