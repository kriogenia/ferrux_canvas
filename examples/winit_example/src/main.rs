use std::thread::sleep;
use std::time::Duration;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::canvas::winit::WinitCanvas;
use ferrux_canvas::color::{Color, ColorBuilder, palette};

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

                canvas.draw_triangle((100, 100 - x as u32), (100 - x as u32, 100),
                                     (200 - x as u32, 200 - x as u32), palette::WHITE);

                canvas.draw_line((200 - x as u32, 100), (200 - x as u32, 200 - x as u32),
                                 palette::RED);
                canvas.draw_line((200 - x as u32, 200 - x as u32), (100, 200 - x as u32),
                                 Color::from_rgba("00ff00ff").unwrap());
                canvas.draw_line((100, 200 - x as u32), (200 - x as u32, 100),
                ColorBuilder::new().with_red(0).with_green(0).with_blue(255).build());

                canvas.fill_triangle((250, 250), (200, 300), (300, 300), palette::GREEN);

                canvas.render().unwrap();
                canvas.reset_frame();
                sleep(Duration::new(5, 0));
            }
            _ => (),
        }
    });

}
