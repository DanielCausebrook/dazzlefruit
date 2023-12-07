use crate::pattern_builder::component::shared_component::SharedComponent;
use crate::pattern_builder::component::texture::Texture;

#[derive(Clone)]
pub struct SingleTextureGenerator<T: Texture + Clone>  {
    texture: SharedComponent<T>,
}
