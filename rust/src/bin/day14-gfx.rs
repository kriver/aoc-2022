use aoc_2022::day14::{Action, Cave, Coord, Type};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

type Color = (u8, u8, u8);

struct CaveGfx {
    pixels: Pixels,
    cave: Cave,
    y: i32,
    sand: Coord,
    done: bool,
}

impl CaveGfx {
    const WIDTH: i32 = 100;
    const HEIGHT: i32 = 100;
    const DX: i32 = 440;
    const FLOOR_DY: i32 = 2;

    const SKY: Color = (0x87, 0xce, 0xeb);
    const ROCK: Color = (0x60, 0x60, 0x60);
    const FLOOR: Color = (0x40, 0x40, 0x40);
    const SAND: Color = (0xed, 0xc9, 0xaf);
    const FALLING: Color = (0xff, 0x40, 0x40);

    fn new(surface_texture: SurfaceTexture<Window>) -> Self {
        CaveGfx {
            pixels: Pixels::new(
                CaveGfx::WIDTH as u32,
                CaveGfx::HEIGHT as u32,
                surface_texture,
            )
            .unwrap(),
            cave: Cave::load(true),
            y: 0,
            sand: Coord::new(500, 0),
            done: false,
        }
    }

    fn scroll(&mut self, dy: f32) {
        if dy < 0. {
            self.y = 0.max(self.y - 10);
        } else {
            self.y = (self.cave.lowest() - CaveGfx::HEIGHT + 10).min(self.y + 10);
        }
    }

    fn animate(&mut self) {
        if !self.done {
            loop {
                match self.cave.sandfall_single_step(self.sand.clone()) {
                    Action::Done => {
                        println!("DONE");
                        self.done = true;
                        break;
                    }
                    Action::Falling(s) => self.sand = s,
                    Action::Settled(settled, s) => {
                        self.y = settled.y();
                        self.sand = s;
                        break;
                    }
                }
            }
        }
    }

    fn draw(&mut self) {
        fn color_pixel(fb: &mut [u8], x: i32, y: i32, c: &Color) {
            let offset = (4 * (CaveGfx::WIDTH * y + x)) as usize;
            fb[offset + 0] = c.0;
            fb[offset + 1] = c.1;
            fb[offset + 2] = c.2;
            fb[offset + 3] = 0xff;
        }

        let base_y = 0.max(self.y - 50);
        let fb = self.pixels.get_frame_mut();
        for y in 0..CaveGfx::HEIGHT {
            for x in 0..CaveGfx::WIDTH {
                if base_y + y >= self.cave.lowest() + CaveGfx::FLOOR_DY {
                    color_pixel(fb, x, y, &CaveGfx::FLOOR);
                } else {
                    match self.cave.grid_at(CaveGfx::DX + x, base_y + y) {
                        None => color_pixel(fb, x, y, &CaveGfx::SKY),
                        Some(Type::Rock) => color_pixel(fb, x, y, &CaveGfx::ROCK),
                        Some(Type::Sand) => color_pixel(fb, x, y, &CaveGfx::SAND),
                    }
                }
            }
        }
        if self.sand.y() >= base_y {
            color_pixel(
                fb,
                self.sand.x() - CaveGfx::DX,
                0.max(self.sand.y() - base_y),
                &CaveGfx::FALLING,
            );
        }
        self.pixels.render();
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sandfall")
        .with_inner_size(PhysicalSize::new(8 * CaveGfx::WIDTH, 8 * CaveGfx::HEIGHT))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut cave = CaveGfx::new(surface_texture);

    event_loop.run(move |event, _target, control_flow| {
        control_flow.set_poll();
        cave.animate();
        window.request_redraw();
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                cave.draw();
            }
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseWheel { delta } => match delta {
                    MouseScrollDelta::LineDelta(_, dy) => {
                        cave.scroll(dy);
                        window.request_redraw();
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    })
}
