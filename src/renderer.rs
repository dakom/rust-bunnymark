use super::state::{State};

use std::rc::Rc;
use std::cell::RefCell;

use web_sys::{HtmlImageElement};
use nalgebra::{Matrix4, Vector3, Vector2, DimMul};

use awsm_web::webgl::{
    BufferMask,
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
    ShaderType,
};

pub struct SceneRenderer {
    renderer: Rc<RefCell<WebGl1Renderer>>,
    ids: SceneIds,
}

struct SceneIds {
    program_id: Id,
    texture_id: Id,
    tex_buffer_id: Id,
    geom_buffer_id: Id
}
impl SceneRenderer {
    pub fn new (webgl_renderer:Rc<RefCell<WebGl1Renderer>>, vertex:&str, fragment:&str, img:&HtmlImageElement) -> Result<Self, awsm_web::errors::Error> {
        let ids = {
            let mut renderer = webgl_renderer.borrow_mut();
            //compile the shaders and get a program id
            let vertex_id = renderer.compile_shader(vertex, ShaderType::Vertex)?;
            let fragment_id = renderer.compile_shader(fragment, ShaderType::Fragment)?;
            let program_id = renderer.compile_program(&[vertex_id, fragment_id])?;


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


            //buffer ids
            let geom_buffer_id = renderer.create_buffer()?;
            let tex_buffer_id = renderer.create_buffer()?;


            SceneIds {program_id, texture_id, tex_buffer_id, geom_buffer_id}
        };

        Ok(Self { renderer: webgl_renderer, ids} )
    }

    pub fn update_uvs(&mut self, state:&State) -> Result<(), awsm_web::errors::Error> {
        println!("{}", state.uvs.len());
        let mut renderer = self.renderer.borrow_mut();
        renderer.upload_buffer_to_attribute_name(
            self.ids.tex_buffer_id,
            BufferData::new(
                &state.uvs,
                BufferTarget::ArrayBuffer,
                BufferUsage::DynamicDraw,
                ),
                "a_tex_uv",
                &AttributeOptions::new(2, DataType::Float),
                )?;
        Ok(())
    }
    pub fn update_vertices(&mut self, state:&State) -> Result<(), awsm_web::errors::Error> {
        let mut renderer = self.renderer.borrow_mut();
        renderer.upload_buffer_to_attribute_name(
            self.ids.geom_buffer_id,
            BufferData::new(
                &state.vertices,
                BufferTarget::ArrayBuffer,
                BufferUsage::DynamicDraw,
                ),
                "a_geom_vertex",
                &AttributeOptions::new(2, DataType::Float),
                )?;
        Ok(())
    }
    pub fn render(&mut self, state:&State) -> Result<(), awsm_web::errors::Error> {
        //if no bunnies, skip rendering
        if state.bunnies.len() == 0 {
            return Ok(())
        }

        let mut renderer = self.renderer.borrow_mut();
        let SceneIds {program_id, texture_id, ..} = self.ids;


        //Clear the screen buffers
        renderer.clear(&[
                BufferMask::ColorBufferBit,
                BufferMask::DepthBufferBit,
        ]);

        //set blend mode. this will be a noop internally if already set
        renderer.toggle(GlToggle::Blend, true);
        renderer.toggle(GlToggle::DepthTest, false);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);

        //will already be activated but internally that's a noop if true
        renderer.activate_program(program_id)?;

        //enable texture
        renderer.activate_texture_for_sampler_name(texture_id, "u_sampler")?;

        //Build our camera matrix (must cast to f32)
        let camera_mat = Matrix4::new_orthographic( 0.0, state.stage_size.width as f32, 0.0, state.stage_size.height as f32, 0.0, 1.0);

        //Upload data to the GPU
        renderer.upload_uniform_mat_4_name("u_camera", &camera_mat.as_slice())?;



        renderer.draw_arrays(BeginMode::Triangles, 0, (state.vertices.len() / 2) as u32);

        Ok(())
    }

}
