use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};

use crate::viewport::Viewport;

#[derive(Debug)]
pub struct App {
	event_loop: EventLoop<()>,
	window: Window,
	viewport: Viewport,
}

impl App {
	// TODO: Add WindowConfig struct
	pub async fn new(window_name: &str) -> Self {
		let event_loop = EventLoop::new();

		let window = WindowBuilder::new().build(&event_loop).unwrap();
		window.set_title(format!("{}: {:?}", window_name, window.id()).as_str());

		let viewport = Viewport::new(&window).await;

		Self {
			event_loop,
			window,
			viewport,
		}
	}

	pub fn run(mut self) {
		self.event_loop.run(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;

			match event {
				Event::WindowEvent { ref event, .. } => {
					match event {
						WindowEvent::CursorMoved { position, .. } => {
							self.viewport.cursor_pos = *position;
						}
						WindowEvent::CloseRequested
						| WindowEvent::KeyboardInput {
							input:
								winit::event::KeyboardInput {
									state: winit::event::ElementState::Pressed,
									virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
									..
								},
							..
						} => *control_flow = ControlFlow::Exit,
						WindowEvent::Resized(physical_size) => {
							self.viewport.resize(*physical_size);
						}
						WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
							self.viewport.resize(**new_inner_size)
						}
						_ => (),
					};
				}
				Event::RedrawRequested(_window_id) => {
					match self.viewport.render() {
						Ok(_) => (),
						Err(wgpu::SurfaceError::Lost) => self.viewport.resize(self.viewport.size),
						Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
						Err(e) => eprintln!("{:?}", e),
					};
				}
				Event::MainEventsCleared => {
					self.window.request_redraw();
				}
				_ => (),
			}
		});
	}
}