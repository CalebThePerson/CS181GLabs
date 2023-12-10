use engine::{Engine, GPUCamera, GPUSprite, Game};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
    window::Window,
};
struct TestGame {
    camera: GPUCamera,
}

#[async_trait::async_trait]
impl Game for TestGame {
    async fn init(&mut self, engine: &mut Engine) {
        //Creating our background image texture, by calling load texture
        let (img, mut imgpng) = engine
            .load_texture(
                "/Users/calebtheperson/RustProjects/somuchpain/scene2d/src/assets/background.jpg",
                None,
            )
            .await
            .expect("Couldn't load background");

        //Then we are adding this and behind the scenes it shoudl be creating a bind group and etc to display it.
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &img,
            vec![GPUSprite {
                screen_region: [0.0, 0.0, 1024.0, 768.0],
                sheet_region: [0.0, 0.0, 1.0, 1.0],
            }],
            self.camera,
        );

        //Same thing but for our sprite
        let (tex_king, mut kingtex_kingpng) = engine
            .load_texture(
                "/Users/calebtheperson/RustProjects/somuchpain/scene2d/src/king.png",
                None,
            )
            .await
            .expect("Couldn't load king img");

        //This sprite-group we would want to add obstacles and etc
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                GPUSprite {
                    screen_region: [192.0, 192.0, 64.0, 64.0],
                    sheet_region: [0.0, 0.0, 0.5, 0.5], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [192.0 + 64.0, 192.0, 64.0, 64.0],
                    sheet_region: [0.0, 0.0, 0.5, 0.5], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [192.0 + 128.0, 192.0, 64.0, 64.0],
                    sheet_region: [0.0, 0.0, 0.5, 0.5], //[0.0, 0.5, 0.5, 0.5]
                },
                GPUSprite {
                    screen_region: [192.0 + 192.0, 192.0, 64.0, 64.0],
                    sheet_region: [0.0, 0.0, 0.5, 0.5], //[0.0, 0.5, 0.5, 0.5]
                },
            ],
            self.camera,
        );

        //This sprite group adds the left Player
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                //It's the 2 different sprites for king.png at 2 different locations
                GPUSprite {
                    screen_region: [32.0, 96.0, 64.0, 64.0],
                    sheet_region: [0.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
                },
            ],
            self.camera,
        );

        //This sprite group adds the right player
        engine.sprites.add_sprite_group(
            &engine.gpu,
            &tex_king,
            vec![
                //It's the 2 different sprites for king.png at 2 different locations
                GPUSprite {
                    screen_region: [384.0, 96.0, 64.0, 64.0],
                    sheet_region: [0.0, 16.0 / 32.0, 16.0 / 32.0, 16.0 / 32.0],
                },
            ],
            self.camera,
        );
    }
    fn update(&mut self, engine: &mut Engine) {
        engine.sprites.set_camera_all(&engine.gpu, self.camera);
        engine
            .sprites
            .refresh_sprites(&engine.gpu, 0, 0..(engine.sprites.get_sprites(0).len()));
    }
}

fn main() {
    let camera = GPUCamera {
        screen_pos: [0.0, 0.0],
        screen_size: [1024.0, 768.0],
    };
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    Engine::start(event_loop, window, TestGame { camera });
}
