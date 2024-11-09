
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture};



use crate::utilities::Utilities;


pub struct Window;

impl Window {
    pub fn render(){
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("rust Window test")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .unwrap();
    
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();
    
        // Fill the window with red
        let width = window_size.width;
        let height: u32 = window_size.height;
        for (i,pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            // let physical_size = window.inner_size();
            // println!("Physical size: {}x{}", physical_size.width, physical_size.height);
    
            let x = (i % width as usize) as u32;
            let y = (i / width as usize) as u32;
    
            pixel[0] = (Utilities::normalize(x, 0, width) * 255.0) as u8; // R
            pixel[1] = (Utilities::normalize(y, 0, height) * 255.0) as u8;   // G
            pixel[2] = 0;   // B
            pixel[3] = 255; // A
        }
    
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
    
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::RedrawRequested(_) => {
                    pixels.render().unwrap();
                }
                _ => (),
            }
        });
    }
}