use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

use futures::executor::block_on;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();


    // Since main can't be async, we're going to need to block
    let mut state = block_on(State::new(&window));


    event_loop.run(move |event, _, control_flow| {
        match event {

            Event::RedrawRequested(_) => {
                state.update();
                state.render();
            }

            Event::MainEventsCleared => {
               window.request_redraw();
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !state.input(event) {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so we have to dereference it twice
                        state.resize(**new_inner_size);
                    }


                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

struct State {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,


    render_pipeline: wgpu::RenderPipeline,

    size: winit::dpi::PhysicalSize<u32>,

    mouse_x: i32
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(window) };
//        let surface = wgpu::Surface::create(window);//Our window needs to implement raw-window-handle 's HasRawWindowHandle trait to access the native window implementation for wgpu to properly create the graphics backend. Fortunately, winit's Window fits the bill

//        let adapter = wgpu::Adapter::request(
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
//            wgpu::BackendBit::PRIMARY, // Vulkan + Metal + DX12 + Browser WebGPU
        ).await.unwrap(); // Get used to seeing this

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            // extensions: wgpu::Extensions {
            //     anisotropic_filtering: false,
            // },
            // limits: Default::default(),
                //copied from wgpu-rs/examples/hello-triangle/main.rs
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                shader_validation: true,
        }, None).await.expect("Failed to create device");

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
let mouse_x = 0;


        let vs_src = include_str!("shader.vert");
        let fs_src = include_str!("shader.frag");

        let mut compiler = shaderc::Compiler::new().unwrap();
        let vs_spirv = compiler.compile_into_spirv(vs_src, shaderc::ShaderKind::Vertex, "shader.vert", "main", None).unwrap();
        let fs_spirv = compiler.compile_into_spirv(fs_src, shaderc::ShaderKind::Fragment, "shader.frag", "main", None).unwrap();

        // let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
        // let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();

        // let vs_module = device.create_shader_module(&vs_data);
        // let fs_module = device.create_shader_module(&fs_data);
            let vs_module = device.create_shader_module(
                wgpu::ShaderModuleSource::SpirV(std::borrow::Cow::Borrowed(vs_spirv.as_binary())));
            let fs_module = device.create_shader_module(
                wgpu::ShaderModuleSource::SpirV(std::borrow::Cow::Borrowed(fs_spirv.as_binary())));
//            let fs_module = device.create_shader_module(fs_spirv.as_binary_u8() as wgpu::ShaderModuleSource);

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {        
            label: None,
            push_constant_ranges: &[],

            bind_group_layouts: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main", // 1.
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor { // 2.
                module: &fs_module,
                entry_point: "main",
            }),

            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
    clamp_depth: device.features().contains(wgpu::Features::DEPTH_CLAMPING),        }),


            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                },
            ],

                    
            primitive_topology: wgpu::PrimitiveTopology::TriangleList, // 1.
            depth_stencil_state: None, // 2.
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16, // 3.
                vertex_buffers: &[], // 4.
            },
            sample_count: 1, // 5.
            sample_mask: !0, // 6.
            alpha_to_coverage_enabled: false, // 7.

         
        });


        Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,

            render_pipeline,
            size,
            mouse_x
        }
 

 
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    // input() won't deal with GPU code, so it can be synchronous
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved{device_id, position, modifiers} => {
                println!("curosro{:?}", position);
                self.mouse_x = position.x;
       //         self.window.request_redraw();
                true
            }
            _ => false
        }
    }

    fn update(&mut self) {
       // unimplemented!()
    }

    fn render(&mut self) {
        let frame = self.swap_chain//.get_next_texture().expect("Timeout getting texture");
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,

                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                // wgpu::Color::GREEN
                                wgpu::Color {
                                    r: (self.mouse_x as f64 / self.size.width as f64),
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                },
                                ),
                            store: true,
                        },
                        // load_op: wgpu::LoadOp::Clear,
                        // store_op: wgpu::StoreOp::Store,
                        
                        // clear_color: 
                        //wgpu::Color {
                        //     r: (self.mouse_x as f64 / self.size.width as f64),
                        //     g: 0.2,
                        //     b: 0.3,
                        //     a: 1.0,
                        // },
                    }
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));

        // self.queue.submit(&[
        //     encoder.finish()
        // ]);
    }
}

