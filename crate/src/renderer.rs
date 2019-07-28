use super::data::{Area, QUAD_GEOM_UNIT};
use super::bunny::{Bunny};
use super::state::{State};

use std::rc::Rc;
use std::cell::RefCell;

use web_sys::{HtmlImageElement};
use nalgebra::{Matrix4, Point2, Vector3};

use awsm::webgl::{
    ClearBufferMask,
    WebGlCommon,
    WebGl1Renderer,
    AttributeOptions,
    BufferData,
    BufferTarget,
    BufferUsage,
    DataType,
    TextureTarget,
    PixelFormat,
    SimpleTextureOptions,
    WebGlTextureSource,
    Id,
    BeginMode,
    GlToggle,
    BlendFactor,
};
use awsm::errors::Error;

pub struct SceneRenderer {
    renderer: Rc<RefCell<WebGl1Renderer>>,
    ids: SceneIds,
}

struct SceneIds {
    program_id: Id,
    geom_id: Id,
    texture_id: Id,
    instance_id: Id,
}
impl SceneRenderer {
    pub fn new (webgl_renderer:Rc<RefCell<WebGl1Renderer>>, vertex:&str, fragment:&str, img:&HtmlImageElement) -> Result<Self, Error> {
        let ids = {
            let mut renderer = webgl_renderer.borrow_mut();
            //This demo is specifically using webgl1, which needs to register the extension
            //Everything else is the same API as webgl2 :)
            renderer.register_extension_instanced_arrays()?;

            //compile the shaders and get a program id
            let program_id = renderer.compile_program(vertex, fragment)?;

            //create quad data and get a buffer id
            let geom_id = renderer.create_buffer()?;

            renderer.upload_buffer_to_attribute(
                geom_id,
                BufferData::new(
                    &QUAD_GEOM_UNIT,
                    BufferTarget::ArrayBuffer,
                    BufferUsage::StaticDraw,
                    ),
                    "a_vertex",
                    &AttributeOptions::new(2, DataType::Float),
                    )?;

            //create texture data and get a texture id
            let texture_id = renderer.create_texture()?;
            renderer.assign_simple_texture(
                texture_id,
                TextureTarget::Texture2d,
                &SimpleTextureOptions {
                    pixel_format: PixelFormat::Rgba,
                    ..SimpleTextureOptions::default()
                },
                &WebGlTextureSource::ImageElement(&img),
                )?;

            //create an instance buffer and get the id
            let instance_id = renderer.create_buffer()?;

            SceneIds {program_id, geom_id, texture_id, instance_id }
        };

        Ok(Self { renderer: webgl_renderer, ids} )
    }

    pub fn render(&mut self, state:&State) -> Result<(), Error> {
        //if no bunnies, skip rendering
        if state.bunnies.len() == 0 {
            return Ok(())
        }

        let mut renderer = self.renderer.borrow_mut();
        let SceneIds {program_id, texture_id, instance_id, geom_id } = self.ids;


        //Clear the screen buffers
        renderer.clear(&[
                ClearBufferMask::ColorBufferBit,
                ClearBufferMask::DepthBufferBit,
        ]);

        //set blend mode. this will be a noop internally if already set
        renderer.toggle(GlToggle::Blend, true);
        renderer.toggle(GlToggle::DepthTest, false);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);

        //will already be activated but internally that's a noop if true
        renderer.activate_program(program_id)?;

        //enable texture
        renderer.activate_texture_for_sampler(texture_id, "u_sampler")?;

        //Build our matrices (must cast to f32)
        let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(state.img_size.width as f32, state.img_size.height as f32, 0.0));
        let camera_mat = Matrix4::new_orthographic( 0.0, state.stage_size.width as f32, 0.0, state.stage_size.height as f32, 0.0, 1.0);

        //Upload them to the GPU
        renderer.upload_uniform_mat_4("u_size", &scaling_mat.as_slice())?;
        renderer.upload_uniform_mat_4("u_camera", &camera_mat.as_slice())?;


    //need the location for the attrib_divisor below
        let loc = renderer.get_attribute_location_value("a_position")?;
        renderer.upload_buffer( instance_id, BufferData::new(
                &state.instance_positions,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
        ))?;

        renderer.activate_attribute_loc(loc, &AttributeOptions::new(2, DataType::Float));

        renderer.vertex_attrib_divisor(loc, 1)?;
        renderer.draw_arrays_instanced(BeginMode::TriangleStrip, 0, 4, state.bunnies.len() as u32)?;

        Ok(())
    }

}
