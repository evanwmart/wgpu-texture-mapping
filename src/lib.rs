// lib.rs

// Standard library imports
use std::iter;

// Imports from the winit crate for windowing and event handling
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{ KeyCode, PhysicalKey },
    window::{ Window, WindowBuilder },
};

// Image processing import
// use image::GenericImageView;

use wgpu::util::DeviceExt;

// Import for WebAssembly (wasm32) target, if applicable
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// The main state struct which holds all resources needed for rendering
struct State<'a> {
    surface: wgpu::Surface<'a>, // Surface that represents the part of the window where rendering occurs
    device: wgpu::Device, // Represents the GPU and handles resource management
    queue: wgpu::Queue, // Handles the submission of commands to the GPU
    config: wgpu::SurfaceConfiguration, // Configuration for the surface, including display format and resolution
    size: winit::dpi::PhysicalSize<u32>, // Window size in physical pixels
    window: &'a Window, // Reference to the window instance for rendering
    render_pipeline: wgpu::RenderPipeline, // The pipeline object that contains rendering configurations
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    rotation_angle_x: f32,
    rotation_angle_y: f32,
}

// Implementation of the State struct
impl<'a> State<'a> {
    // Asynchronous method to initialize a new State instance
    async fn new(window: &'a Window) -> State<'a> {
        let size = window.inner_size(); // Get the initial window size

        // Create an instance for interfacing with the GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY, // Use primary backend on native platforms
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL, // Use OpenGL backend for WebAssembly
            ..Default::default()
        });

        // Create a surface for rendering in the window
        let surface = instance.create_surface(window).unwrap();

        // Request a GPU adapter that meets the preferred criteria
        let adapter = instance
            .request_adapter(
                &(wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance, // Prefer high-performance GPU
                    compatible_surface: Some(&surface), // Ensure adapter is compatible with the surface
                    force_fallback_adapter: false, // Do not force a fallback adapter
                })
            ).await
            .expect("Failed to find a compatible GPU adapter");

        // Request a logical device and a command queue from the adapter
        let (device, queue) = adapter
            .request_device(
                &(wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                }),
                None
            ).await
            .unwrap();

        // Get the supported formats and modes for the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats
            .iter()
            .copied()
            .find(|f| f.is_srgb()) // Prefer sRGB format for better color accuracy
            .unwrap_or(surface_caps.formats[0]);

        // Configure the surface with specified usage and format
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Usage for render output
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2, // Or Some(value)
        };

        // Configure the surface with device and configuration
        surface.configure(&device, &config);

        // ** NEW CODE STARTS HERE **

        // Import image crate for loading PNG files
        use image::GenericImageView; // Add this import at the top of your file

        // Load the image
        let img = image::open("assets/scenary.png").expect("Failed to load texture");
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        // Create the texture
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &(wgpu::TextureDescriptor {
                label: Some("Texture"),
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
        );

        // Copy the image data into the texture
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: None,
            },
            texture_size
        );

        // Create a texture view
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a sampler
        let sampler = device.create_sampler(
            &(wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            })
        );

        // ** NEW CODE ENDS HERE **

        // Load the WGSL shader code from an external file
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // let transform_matrix = glam::Mat4::IDENTITY.to_cols_array();
        // let uniform_buffer = device.create_buffer_init(
        //     &(wgpu::util::BufferInitDescriptor {
        //         label: Some("Uniform Buffer"),
        //         contents: bytemuck::cast_slice(&transform_matrix),
        //         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        //     })
        // );

        let transform_matrix = [0.0f32; 16]; // 4x4 matrix
        let uniform_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&transform_matrix),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
        );

        // ** UPDATED BIND GROUP LAYOUT **

        let bind_group_layout = device.create_bind_group_layout(
            &(wgpu::BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
                entries: &[
                    // Transformation matrix
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Texture binding
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Sampler binding
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            })
        );

        // ** UPDATED BIND GROUP **

        let bind_group = device.create_bind_group(
            &(wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: Some("Bind Group"),
            })
        );

        // Set up the render pipeline layout
        let render_pipeline_layout = device.create_pipeline_layout(
            &(wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            })
        );

        // Create the render pipeline
        let render_pipeline = device.create_render_pipeline(
            &(wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING), // Use alpha blending if your texture has transparency
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            })
        );

        // Return the initialized State
        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            uniform_buffer,
            bind_group,
            rotation_angle_x: 0.0,
            rotation_angle_y: 0.0,
        }
    }

    // Accessor for the window reference
    fn window(&self) -> &Window {
        &self.window
    }

    // Resize handler to update surface configuration if the window size changes
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    // Handles input events, returning false as no input handling is done in this example
    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    // Update function to rotate the square in 3D space
    fn update(&mut self) {
        // Update rotation angles with different speeds
        self.rotation_angle_x += 0.0001; // Speed for X-axis rotation
        self.rotation_angle_y += 0.0003; // Speed for Y-axis rotation
    
        // Create rotation matrices
        let rotation_x = glam::Mat4::from_rotation_x(self.rotation_angle_x);
        let rotation_y = glam::Mat4::from_rotation_y(self.rotation_angle_y);
    
        // Combine rotations
        let model = rotation_y * rotation_x;
    
        // View matrix
        let eye = glam::Vec3::new(0.0, 0.0, 2.0);
        let center = glam::Vec3::ZERO;
        let up = glam::Vec3::Y;
        let view = glam::Mat4::look_at_rh(eye, center, up);
    
        // Projection matrix
        let aspect_ratio = self.size.width as f32 / self.size.height as f32;
        let fovy = 45.0f32.to_radians();
        let projection = glam::Mat4::perspective_rh(fovy, aspect_ratio, 0.1, 100.0);
    
        // MVP matrix
        let mvp = projection * view * model;
    
        // Update the uniform buffer
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&mvp.to_cols_array()),
        );
    }
    

    // Render function that performs the drawing operations
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?; // Get the next texture for rendering
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default()); // Create a view for the texture

        let mut encoder = self.device.create_command_encoder(
            &(wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            })
        );

        // Start the render pass
        let mut render_pass = encoder.begin_render_pass(
            &(wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.02,
                                g: 0.02,
                                b: 0.05,
                                a: 1.0, // Background color for clearing the screen
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            })
        );

        render_pass.set_pipeline(&self.render_pipeline); // Set the render pipeline
        render_pass.set_bind_group(0, &self.bind_group, &[]); // Bind the uniform buffer
        render_pass.draw(0..6, 0..1); // Draw 6 vertices for two triangles

        drop(render_pass); // End the render pass

        self.queue.submit(iter::once(encoder.finish())); // Submit the command buffer for execution
        output.present(); // Present the rendered image to the window

        Ok(())
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        use winit::platform::web::WindowExtWebSys;

        web_sys
            ::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");

        let _ = window.request_inner_size(PhysicalSize::new(450, 400));
    }

    let mut state = State::new(&window).await;

    event_loop
        .run(move |event, control_flow| {
            // Use state's window reference instead of moving the original window
            state.window().request_redraw();

            match event {
                Event::WindowEvent { ref event, window_id } if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
                            | WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                  event: KeyEvent {
                                      state: ElementState::Pressed,
                                      physical_key: PhysicalKey::Code(KeyCode::Escape),
                                      ..
                                  },
                                  ..
                              } => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            WindowEvent::RedrawRequested => {
                                state.update(); // This will now be called on each redraw
                                match state.render() {
                                    Ok(_) => {}
                                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) =>
                                        state.resize(state.size),
                                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                                    Err(wgpu::SurfaceError::Timeout) =>
                                        log::warn!("Surface timeout"),
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}
