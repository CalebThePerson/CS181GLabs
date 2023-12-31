use crate::{input, sprite::SpriteRender, GPUCamera, Game, WGPU};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
    window::Window,
};
pub struct Engine {
    pub gpu: WGPU,
    pub sprites: SpriteRender,
    pub input: input::Input,
}

impl Engine {
    pub fn start(event_loop: EventLoop<()>, window: Window, game: impl Game + 'static) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
            // On native, we just want to wait for `run` to finish.
            pollster::block_on(Self::run(event_loop, window, game));
        }
        #[cfg(target_arch = "wasm32")]
        {
            // On web things are a little more complicated.
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            use winit::platform::web::WindowExtWebSys;
            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
            // Now we use the browser's runtime to spawn our async run function.
            wasm_bindgen_futures::spawn_local(run(event_loop, window));
        }
    }
    async fn run(event_loop: EventLoop<()>, window: Window, mut game: impl Game + 'static) {
        let mut gpu = WGPU::new(&window).await;
        let mut sprites = SpriteRender::new(&gpu);

        let input = input::Input::default();
        let mut engine = Engine {
            gpu,
            sprites,
            input,
        };

        game.init(&mut engine).await;
        event_loop.run(move |event, _, control_flow| {
            // By default, tell the windowing system that there's no more work to do
            // from the application's perspective.
            *control_flow = ControlFlow::Wait;
            // Depending on the event, we'll need to do different things.
            // There is some pretty fancy pattern matching going on here,
            // so think back to CSCI054.
            match event {
                Event::WindowEvent {
                    // For example, "if it's a window event and the specific window event is that
                    // we have resized the window to a particular new size called `size`..."
                    event: WindowEvent::Resized(size),
                    // Ignoring the rest of the fields of Event::WindowEvent...
                    ..
                } => {
                    // Reconfigure the surface with the new size
                    engine.gpu.resize(size);
                    // On MacOS the window needs to be redrawn manually after resizing
                    window.request_redraw();
                }
                Event::WindowEvent {
                    // Note this deeply nested pattern match
                    event: WindowEvent::KeyboardInput { input: key_ev, .. },
                    ..
                } => {
                    engine.input.handle_key_event(key_ev);
                }

                Event::RedrawRequested(_) => {
                    //This is all the code for moving the left side player
                    if (engine.input.is_key_down(winit::event::VirtualKeyCode::W)) {
                        //Technically 0 Should always be the background
                        //2 should always be the sprite until i change it
                        let oldRegion = engine.sprites.get_sprites(2)[0].screen_region;
                        let mut newRegion = [
                            oldRegion[0],
                            oldRegion[1] + 32.0,
                            oldRegion[2],
                            oldRegion[3],
                        ];
                        engine.sprites.update_position(newRegion, 2);
                    }

                    if (engine.input.is_key_down(winit::event::VirtualKeyCode::S)) {
                        //Technically 0 Should always be the background
                        //2 should always be the sprite until i change it
                        let oldRegion = engine.sprites.get_sprites(2)[0].screen_region;
                        let mut newRegion = [
                            oldRegion[0],
                            oldRegion[1] - 32.0,
                            oldRegion[2],
                            oldRegion[3],
                        ];
                        engine.sprites.update_position(newRegion, 2);
                    }
                    if (engine.input.is_key_down(winit::event::VirtualKeyCode::D)) {
                        //Technically 0 Should always be the background
                        //2 should always be the sprite until i change it
                        let oldRegion = engine.sprites.get_sprites(2)[0].screen_region;
                        let mut newRegion = [
                            oldRegion[0] + 32.0,
                            oldRegion[1],
                            oldRegion[2],
                            oldRegion[3],
                        ];
                        engine.sprites.update_position(newRegion, 2);
                    }
                    if (engine.input.is_key_down(winit::event::VirtualKeyCode::A)) {
                        //Technically 0 Should always be the background
                        //2 should always be the sprite until i change it
                        let oldRegion = engine.sprites.get_sprites(2)[0].screen_region;
                        let mut newRegion = [
                            oldRegion[0] - 32.0,
                            oldRegion[1],
                            oldRegion[2],
                            oldRegion[3],
                        ];
                        engine.sprites.update_position(newRegion, 2);
                    }

                    //This is all code for moving the Right side Player
                    if (engine.input.is_key_down(winit::event::VirtualKeyCode::Up)) {
                        //Technically 0 Should always be the background
                        //2 should always be the sprite until i change it
                        let oldRegion = engine.sprites.get_sprites(3)[0].screen_region;
                        let mut newRegion = [
                            oldRegion[0],
                            oldRegion[1] + 32.0,
                            oldRegion[2],
                            oldRegion[3],
                        ];
                        engine.sprites.update_position(newRegion, 3);
                    }

                    if (engine.input.is_key_down(winit::event::VirtualKeyCode::Down)) {
                        //Technically 0 Should always be the background
                        //2 should always be the sprite until i change it
                        let oldRegion = engine.sprites.get_sprites(3)[0].screen_region;
                        let mut newRegion = [
                            oldRegion[0],
                            oldRegion[1] - 32.0,
                            oldRegion[2],
                            oldRegion[3],
                        ];
                        engine.sprites.update_position(newRegion, 3);
                    }
                    if (engine
                        .input
                        .is_key_down(winit::event::VirtualKeyCode::Right))
                    {
                        //Technically 0 Should always be the background
                        //2 should always be the sprite until i change it
                        let oldRegion = engine.sprites.get_sprites(3)[0].screen_region;
                        let mut newRegion = [
                            oldRegion[0] + 32.0,
                            oldRegion[1],
                            oldRegion[2],
                            oldRegion[3],
                        ];
                        engine.sprites.update_position(newRegion, 3);
                    }
                    if (engine.input.is_key_down(winit::event::VirtualKeyCode::Left)) {
                        //Technically 0 Should always be the background
                        //2 should always be the sprite until i change it
                        let oldRegion = engine.sprites.get_sprites(3)[0].screen_region;
                        let mut newRegion = [
                            oldRegion[0] - 32.0,
                            oldRegion[1],
                            oldRegion[2],
                            oldRegion[3],
                        ];
                        engine.sprites.update_position(newRegion, 3);
                    }

                    // engine.sprites.platform_move();

                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        1,
                        0..(engine.sprites.get_sprites(0).len()),
                    );

                    //This refreshes the sprite player group to update the position of both sprites
                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        2,
                        0..(engine.sprites.get_sprites(0).len()),
                    );

                    engine.sprites.refresh_sprites(
                        &engine.gpu,
                        3,
                        0..(engine.sprites.get_sprites(0).len()),
                    );

                    game.update(&mut engine);
                    engine.input.next_frame();

                    // If the window system is telling us to redraw, let's get our next swapchain image
                    let frame = engine
                        .gpu
                        .surface
                        .get_current_texture()
                        .expect("Failed to acquire next swap chain texture");
                    // And set up a texture view onto it, since the GPU needs a way to interpret those
                    // image bytes for writing.
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    // From the queue we obtain a command encoder that lets us issue GPU commands
                    let mut encoder = engine
                        .gpu
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        // Now we begin a render pass.  The descriptor tells WGPU that
                        // we want to draw onto our swapchain texture view (that's where the colors will go)
                        // and that there's no depth buffer or stencil buffer.
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: None,
                        });
                        engine.sprites.render(&mut rpass);
                    }

                    // Once the commands have been scheduled, we send them over to the GPU via the queue.
                    engine.gpu.queue.submit(Some(encoder.finish()));
                    // Then we wait for the commands to finish and tell the windowing system to
                    // present the swapchain image.
                    frame.present();

                    // (3)
                    // And we have to tell the window to redraw!
                    window.request_redraw(); // Creates a loop and procedds to redraw the window
                }
                // If we're supposed to close the window, tell the event loop we're all done
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                // Ignore every other event for now.
                _ => {}
            }
        });
    }
    pub async fn load_texture(
        &self,
        path: impl AsRef<std::path::Path>,
        label: Option<&str>,
    ) -> Result<(wgpu::Texture, image::RgbaImage), image::ImageError> {
        self.gpu.load_texture(path.as_ref(), label).await
    }
}
