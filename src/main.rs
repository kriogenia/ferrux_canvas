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

	let mut x: i32 = 1;
	let mut incrementing = true;

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
				if !(1..100).contains(&x) {
					incrementing = !incrementing;
				}
				x += if incrementing { 1 } else { -1 };

				canvas.draw_triangle((100, (100 - x) as u32), (100 - x as u32, 100),
				                     (200 - x as u32, 200 - x as u32));
				canvas.render().unwrap();
				canvas.reset_frame();
			}
			_ => (),
		}
	});

}
