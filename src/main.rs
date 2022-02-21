use std::thread::sleep;
use std::time::Duration;
use log::info;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::canvas::winit::WinitCanvas;

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

	let mut canvas = WinitCanvas::new(&window).unwrap();

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
				sleep(Duration::new(2, 0));
				canvas.clear_frame().unwrap();
				sleep(Duration::new(2, 0));
				canvas.reset_frame();
				canvas.draw_line((100, 100), (300, 300));
				canvas.draw_line((100, 300), (300, 100));
				canvas.render().unwrap();
			}
			_ => (),
		}
	});

}
