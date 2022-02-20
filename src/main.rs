use log::info;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ferrux_canvas::canvas::Canvas;

fn main() {
	println!("FerruX Canvas demo");

	let event_loop = EventLoop::new();

	let window = {
		let size = LogicalSize::new(960, 480);
		WindowBuilder::new()
			.with_title("FerruX Canvas")
			.with_inner_size(size)
			.with_min_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};

	let mut canvas = Canvas::new(&window).unwrap();

	canvas.draw_pixel(175, 75);
	canvas.draw_pixel(125, 75);
	canvas.draw_line((150, 50), (150, 250));
	canvas.draw_triangle((100, 100), (150, 150), (200, 100));

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				info!("The close button was pressed; stopping");
				*control_flow = ControlFlow::Exit
			}
			Event::MainEventsCleared => {
				window.request_redraw();
			}
			Event::RedrawRequested(_) => {
				info!("Render canvas");
				canvas.render().unwrap();
			}
			_ => (),
		}
	});

}
