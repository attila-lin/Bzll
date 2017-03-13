// father of all kinds of renderengine like opengl, dx12
use gfx;

use rand::{self, Rng};

//use super::types::*;
//use super::types::ColorFormat;
use render::types::ColorFormat;
gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "a_Position",
    }

    // color format: 0xRRGGBBAA
    vertex Instance {
        translate: [f32; 2] = "a_Translate",
        color: u32 = "a_Color",
    }

    constant Locals {
        scale: f32 = "u_Scale",
    }

    pipeline pipe {
        vertex: gfx::VertexBuffer<Vertex> = (),
        instance: gfx::InstanceBuffer<Instance> = (),
        scale: gfx::Global<f32> = "u_Scale",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

fn fill_instances(instances: &mut [Instance], instances_per_length: u32, size: f32) {
    let gap = 0.4 / (instances_per_length + 1) as f32;
    println!("gap: {}", gap);

    let begin = -1. + gap + (size / 2.);
    let mut translate = [begin, begin];
    let mut rng = rand::StdRng::new().unwrap();

    let length = instances_per_length as usize;
    for x in 0..length {
        for y in 0..length {
            let i = x * length + y;
            instances[i] = Instance {
                translate: translate,
                color: rng.next_u32(),
            };
            translate[1] += size + gap;
        }
        translate[1] = begin;
        translate[0] += size + gap;
    }
}

const MAX_INSTANCE_COUNT: usize = 2048;

pub struct RenderEngine<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
    upload: gfx::handle::Buffer<R, Instance>,
    uploading: bool, // TODO: not needed if we have the encoder everywhere
}

impl<R> RenderEngine<R>
    where R: gfx::Resources
{
    fn new<F>(factory: &mut F,
              color_format: gfx::handle::RenderTargetView<R,
                  (gfx::format::R8_G8_B8_A8,
                   gfx::format::Unorm)>)
              -> Self
        where F: gfx::Factory<R> + gfx::traits::FactoryExt<R>
    {

        let pso = factory.create_pipeline_simple(include_bytes!("../../artist/shader/instancing_150.glslv"),
                                                 include_bytes!("../../artist/shader/instancing_150.glslf"),
                                                 pipe::new())
            .unwrap();

        let instances_per_length: u32 = 32;
        println!("{} instances per length", instances_per_length);
        let instance_count = instances_per_length * instances_per_length;
        println!("{} instances", instance_count);
        assert!(instance_count as usize <= MAX_INSTANCE_COUNT);
        let size = 1.6 / instances_per_length as f32;
        println!("size: {}", size);

        let upload = factory.create_upload_buffer(instance_count as usize).unwrap();
        {
            let mut writer = factory.write_mapping(&upload).unwrap();
            fill_instances(&mut writer, instances_per_length, size);
        }

        let instances = factory.create_buffer(instance_count as usize,
                                              gfx::buffer::Role::Vertex,
                                              gfx::memory::Usage::Data,
                                              gfx::TRANSFER_DST)
            .unwrap();


        let (quad_vertices, mut slice) =
            factory.create_vertex_buffer_with_slice(&QUAD_VERTICES, &QUAD_INDICES[..]);
        slice.instances = Some((instance_count, 0));
        let locals = Locals { scale: size };

        RenderEngine {
            pso: pso,
            data: pipe::Data {
                vertex: quad_vertices,
                instance: instances,
                scale: size,
                locals: factory.create_buffer_immutable(&[locals],
                                                        gfx::buffer::Role::Constant,
                                                        gfx::Bind::empty())
                    .unwrap(),
                out: color_format,
            },
            slice: slice,
            upload: upload,
            uploading: true,
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        if self.uploading {
            encoder.copy_buffer(&self.upload, &self.data.instance, 0, 0, self.upload.len())
                .unwrap();
            self.uploading = false;
        }

        encoder.clear(&self.data.out, [0.1, 0.2, 0.3, 1.0]);
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}


// TODO move to config
const QUAD_VERTICES: [Vertex; 4] = [Vertex { position: [-0.5, 0.5] },
    Vertex { position: [-0.5, -0.5] },
    Vertex { position: [0.5, -0.5] },
    Vertex { position: [0.5, 0.5] }];

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];
