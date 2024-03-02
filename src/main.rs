extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::time::{Duration, SystemTime};
use std::thread::sleep;

const TEXTURE_SIZE : u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

// 一个用来创建正方形纹理的函数
fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, 
                        texture_creator: &'a TextureCreator<WindowContext>,
                        color: TextureColor,
                        size: u32
                        ) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator
            .create_texture_target(None, size, size) {
                canvas.with_texture_canvas(&mut square_texture, |texture| {
                    match color {
                        TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                        TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                    }
                    texture.clear(); // fill the color
                }).expect("Failed to color a texture");
                Some(square_texture)
            } else {
                None
            }
}

fn main() {
    // 初始化sdl
    let sdl_context = sdl2::init().expect("SDL Init failed");
    // 获取视频系统
    let video_subsystem = sdl_context.video().expect("Couldn't get sdl video subsystem");
    // 获取窗口，并设置窗口的属性，整个屏幕居中，使用opengl渲染
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
                    .position_centered()
                    .opengl()
                    .build()
                    .expect("Failed to create window");
    // 获取窗口画布，支持垂直同步
    let mut canvas = window.into_canvas()
                    .target_texture()
                    .present_vsync()
                    .build()
                    .expect("Failed to convert window into canvas");
    // 获取画布的纹理创建者
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // 创建一个正方形纹理
    let green_square = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Green, TEXTURE_SIZE).expect("Failed to create a texture");
    let blue_square = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TEXTURE_SIZE).expect("Failed to create a texture");

    let timer = SystemTime::now();

    // 事件句柄
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    'running: loop {
        // 事件处理循环
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | 
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => 
                {
                    break 'running // 如果收到esc或关闭，退出这个事件循环
                },
                _=> {}
            }
        }
        // 绘制窗口的背景色
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        // 切换颜色显示
        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => {
                true
            }
        };

        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };

        // 把纹理拷贝到窗口中的指定位置
        canvas.copy(&square_texture, None, Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
                    .expect("Failed to copy texture into window");
        // 更新窗口显示
        canvas.present();

        // 每1秒60帧执行这个循环，所以要没1/60秒就sleep一下
        sleep(Duration::new(0, 1_000_000_000u32/60));
    }
}
