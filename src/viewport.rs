use wgpu::{util::DeviceExt, BufferUsages};
use winit::{dpi::PhysicalPosition, window::Window};

use crate::buffers::Vertex;

const _TRI: &[Vertex] = &[
	Vertex {
		// center top
		position: [0.0, 1.0, 0.0],
		color: [1.0, 0.0, 0.0],
	},
	Vertex {
		// left bottom
		position: [-1.0, -1.0, 0.0],
		color: [0.0, 1.0, 0.0],
	},
	Vertex {
		// right bottom
		position: [1.0, -1.0, 0.0],
		color: [0.0, 0.0, 1.0],
	},
];

const QUAD: &[Vertex] = &[
	Vertex {
		// left top
		position: [-1.0, 1.0, 0.0],
		color: [0.5, 0.5, 0.0],
	},
	Vertex {
		// right bottom
		position: [1.0, -1.0, 0.0],
		color: [0.0, 0.0, 1.0],
	},
	Vertex {
		// right top
		position: [1.0, 1.0, 0.0],
		color: [0.5, 0.0, 0.5],
	},
	Vertex {
		// left top
		position: [-1.0, 1.0, 0.0],
		color: [0.5, 0.5, 0.0],
	},
	Vertex {
		// left bottom
		position: [-1.0, -1.0, 0.0],
		color: [0.0, 1.0, 0.0],
	},
	Vertex {
		// right bottom
		position: [1.0, -1.0, 0.0],
		color: [0.0, 0.0, 1.0],
	},
];

#[derive(Debug)]
pub struct Viewport {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	pub(crate) size: winit::dpi::PhysicalSize<u32>,
	pub(crate) cursor_pos: PhysicalPosition<f64>,
	render_pipeline: wgpu::RenderPipeline,
	vertex_buffer: wgpu::Buffer,
}

impl Viewport {
	pub async fn new(window: &Window) -> Self {
		// The instance is a handle to the GPU
		// Backends::all ==> Vulkan | Metal | DX12 | Browser WebGPU
		let instance = wgpu::Instance::new(wgpu::Backends::all());
		let surface = unsafe { instance.create_surface(window) };

		let adapter = instance
			// TODO: Check all available adapters and pick the best / one that works
			// https://docs.rs/wgpu/latest/wgpu/struct.Adapter.html
			// https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#state-new
			.request_adapter(&wgpu::RequestAdapterOptionsBase {
				power_preference: wgpu::PowerPreference::default(),
				force_fallback_adapter: false,
				compatible_surface: Some(&surface),
			})
			.await
			.unwrap();

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					features: wgpu::Features::empty(),
					limits: wgpu::Limits::default(),
					label: None,
				},
				None,
			)
			.await
			.unwrap();

		let size = window.inner_size();
		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface.get_preferred_format(&adapter).unwrap(),
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
		};

		let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
			label: Some("Solid Color"),
			source: wgpu::ShaderSource::Wgsl(include_str!("test.wgsl").into()),
		});

		let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[],
			push_constant_ranges: &[],
		});

		let render_pipeline_desc = wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[Vertex::layout()],
			},
			fragment: None,
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: Some(wgpu::Face::Back),
				polygon_mode: wgpu::PolygonMode::Fill,
				unclipped_depth: false,
				conservative: false,
			},
			depth_stencil: None,
			multisample: wgpu::MultisampleState::default(),
			multiview: None,
		};

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[wgpu::ColorTargetState {
					format: config.format,
					blend: Some(wgpu::BlendState::REPLACE),
					write_mask: wgpu::ColorWrites::ALL,
				}],
			}),
			..render_pipeline_desc
		});

		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Triangle Vertex Buffer"),
			usage: BufferUsages::VERTEX,
			contents: bytemuck::cast_slice(QUAD),
		});

		Self {
			surface,
			device,
			queue,
			config,
			size,
			cursor_pos: PhysicalPosition::default(),
			render_pipeline,
			vertex_buffer,
		}
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: self.cursor_pos.x / self.size.width as f64,
							g: self.cursor_pos.y / self.size.height as f64,
							b: 1.0,
							a: 1.0,
						}),
						store: true,
					},
				}],
				depth_stencil_attachment: None,
			});

			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
			render_pass.draw(0..QUAD.len() as u32, 0..1);
		}

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}
