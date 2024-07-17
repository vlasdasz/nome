use std::{collections::HashMap, ops::Range};

use gm::{
    checked_usize_to_u32,
    flat::{Point, Size},
    volume::Vertex,
    Color,
};
use refs::Weak;
use wgpu::{Buffer, BufferUsages, PolygonMode, PrimitiveTopology, RenderPass, RenderPipeline, ShaderStages};

use crate::{
    image::Image,
    render::{
        sprite_drawer::shader_data::{SpriteBox, SpriteRenderView},
        uniform::{make_uniform_layout, UniformBind},
        vec_buffer::VecBuffer,
        vertex_layout::VertexLayout,
    },
    utils::DeviceHelper,
    WGPUApp,
};

const VERTICES: [Vertex; 4] = [
    Vertex {
        pos: Point::new(-1.0, 1.0),
        uv:  Point::new(0.0, 0.0),
    },
    Vertex {
        pos: Point::new(-1.0, -1.0),
        uv:  Point::new(0.0, 1.0),
    },
    Vertex {
        pos: Point::new(1.0, 1.0),
        uv:  Point::new(1.0, 0.0),
    },
    Vertex {
        pos: Point::new(1.0, -1.0),
        uv:  Point::new(1.0, 1.0),
    },
];

const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());

#[derive(Debug)]
pub struct TexturedBoxPipeline {
    render_pipeline: RenderPipeline,

    view: UniformBind<SpriteRenderView>,

    vertex_buffer: Buffer,

    instances: HashMap<Weak<Image>, VecBuffer<SpriteBox>>,
}

impl Default for TexturedBoxPipeline {
    fn default() -> Self {
        let device = WGPUApp::device();
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/sprite_textured.wgsl"));

        let sprite_view_layout = make_uniform_layout("sprites_view_layout", ShaderStages::VERTEX_FRAGMENT);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                "textured_sprite_pipeline_layout".into(),
            bind_group_layouts:   &[&sprite_view_layout, &Image::uniform_layout()],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.pipeline(
            "textured_sprite_render_pipeline",
            &pipeline_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Vertex::VERTEX_LAYOUT, SpriteBox::VERTEX_LAYOUT],
        );

        let vertex_buffer = device.buffer(&VERTICES, BufferUsages::VERTEX);

        Self {
            render_pipeline,
            view: sprite_view_layout.into(),
            vertex_buffer,
            instances: HashMap::default(),
        }
    }
}

impl TexturedBoxPipeline {
    pub fn add(
        &mut self,
        image: Weak<Image>,
        size: Size,
        position: Point,
        rotation: f32,
        color: Color,
        z_position: f32,
    ) {
        let image = self.instances.entry(image).or_default();

        image.push(SpriteBox {
            size,
            position,
            color,
            rotation,
            z_position,
        });
    }

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        scale: f32,
        camera_rotation: f32,
        camera_pos: Point,
        resolution: Size,
    ) {
        render_pass.set_pipeline(&self.render_pipeline);

        self.view.update(SpriteRenderView {
            camera_pos,
            resolution,
            camera_rotation,
            scale,
        });

        for (image, instances) in &mut self.instances {
            instances.load();

            render_pass.set_bind_group(0, self.view.bind(), &[]);
            render_pass.set_bind_group(1, &image.bind, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instances.buffer().slice(..));

            render_pass.draw(VERTEX_RANGE, 0..instances.len());
        }
    }
}
