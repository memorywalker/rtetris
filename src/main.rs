extern crate sdl2;

mod score_file;
mod tetris;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadTexture, InitFlag};

use std::time::{Duration, SystemTime};
use std::thread::sleep;

use crate::score_file::print_game_information;
use crate::tetris::update_tetris;
use crate::tetris::Tetris;


// 一个格子的大小
const TETRIS_HEIGHT: usize = 40;

// 一个用来创建正方形纹理的函数
fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, 
    texture_creator: &'a TextureCreator<WindowContext>,
    r: u8, g: u8, b: u8,
    size: u32
    ) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = 
        texture_creator.create_texture_target(None, size, size) {
            canvas.with_texture_canvas(&mut square_texture, |texture| {
                texture.set_draw_color(Color::RGB(r, g, b));
                texture.clear(); // fill the color
                }).expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

fn create_texture_from_text<'a>(texture_creator: &'a TextureCreator<WindowContext>,
       font: &sdl2::ttf::Font,
       text: &str,
       r: u8, g: u8, b: u8) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text).blended(Color::RGB(r, g, b)) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 12, 20))
}

fn display_game_information<'a>(tetris: &Tetris,
       canvas: &mut Canvas<Window>,
       texture_creator: &'a TextureCreator<WindowContext>,
       font: &sdl2::ttf::Font,
       start_x_point: i32) {
     let score_text = format!("Score: {}", tetris.score);
     let lines_sent_text = format!("Lines sent: {}", tetris.nb_lines);
     let level_text = format!("Level: {}", tetris.current_level);

     let score = create_texture_from_text(&texture_creator, &font,
        &score_text, 255, 255, 255)
        .expect("Cannot render text");
     let lines_sent = create_texture_from_text(&texture_creator, &font,
        &lines_sent_text, 255, 255, 255)
        .expect("Cannot render text");
     let level = create_texture_from_text(&texture_creator, &font,
        &level_text, 255, 255, 255)
        .expect("Cannot render text");
     
     canvas.copy(&score, None, get_rect_from_text(&score_text, 
        start_x_point, 75))
          .expect("Couldn't copy text");
    canvas.copy(&lines_sent, None, get_rect_from_text(&score_text,
        start_x_point, 125))
          .expect("Couldn't copy text");
    canvas.copy(&level, None, get_rect_from_text(&score_text, 
       start_x_point, 160))
          .expect("Couldn't copy text");
}

fn main() {
    // 初始化sdl
    let sdl_context = sdl2::init().expect("SDL Init failed");

    // 获取视频系统
    let video_subsystem = sdl_context.video().expect("Couldn't get sdl video subsystem");
    
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("Failed to initialize the image context");
    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization
        failed");
    let mut font = ttf_context.load_font("res/font/Bitter-Regular.ttf", 60).expect("Couldn't load the font");  
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let width = 600;
    let height = 800;
    let grid_x = 20; // 地图开始的左边距
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2; // 地图开始的顶部位置
    // 获取窗口，并设置窗口的属性，整个屏幕居中，使用opengl渲染
    let window = video_subsystem.window("rust-sdl2 demo: Video", width, height)
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

    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();
    
    // 背景图片
    let image_texture = texture_creator.load_texture("res/images/flower.jpeg").expect("Failed to load image");
    // 游戏地图背景
    let grid = create_texture_rect(&mut canvas, &texture_creator, 0, 0, 0 , TETRIS_HEIGHT as u32 * 10).expect("Failed to create a texture");
    // 边框为10像素
    let border = create_texture_rect(&mut canvas, &texture_creator, 255,255, 255, TETRIS_HEIGHT as u32 * 10 + 20).expect("Failed to create a texture");
    // 使用宏简化代码
    macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => (
            create_texture_rect(&mut canvas, &texture_creator, 
                $r, $g, $b, TETRIS_HEIGHT as u32).unwrap()
        )
    }
    // 7种纹理方块，对应每个块的颜色
    let textures = [texture!(255, 69, 69), texture!(255, 220, 69), texture!(237, 150, 37), 
                                        texture!(171, 99, 237), texture!(77, 149, 239), 
                                        texture!(39, 218, 225), texture!(45, 216, 47)];

    // 事件句柄
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    // 绘制窗口的背景色
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();

    loop {
        // 处理下落逻辑数据
        tetris::falling(&mut tetris, &mut timer);

        canvas.copy(&image_texture, None, None).expect("Couldn't copy texture into window");
        // 纹理会被拉伸以适应目标区域大小
        canvas.copy(&border,
        None,
        Rect::new(10,
                  (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2 - 10, // 垂直居中
                  TETRIS_HEIGHT as u32 * 10 + 20, TETRIS_HEIGHT as u32 * 16 + 20))
        .expect("Couldn't copy texture into window");    
        // 游戏区域的黑色背景，用来擦除刷新
        canvas.copy(&grid,
            None,
            Rect::new(20,(height - TETRIS_HEIGHT as u32 * 16) as i32 / 2,TETRIS_HEIGHT as u32 * 10, TETRIS_HEIGHT as u32 * 16))
                 .expect("Couldn't copy texture into window");
        // 如果当前块已经被合并了，创建新一个新的方块开始下落
        if !update_tetris(&mut tetris) {
            break
        }

        let mut quit = false;
        // 处理按键事件，如果按键事件导致方块合并到了网格地图中，就不需要绘制下落的方块了，否则还需要绘制下落的方块
        if !tetris::handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            if let Some(ref mut piece) = tetris.current_piece {
                for (line_nb, line) in piece.states[piece.current_state as usize].iter().enumerate() {
                    for (case_nb, case) in line.iter().enumerate() {
                        // 如果块的状态的格子为0，说明是空的，不用绘制
                        if *case == 0 {
                            continue
                        }
                        // 绘制当前移动的块的一个格子，case为块中的数字，用来选择用那种颜色
                        canvas.copy(&textures[*case as usize - 1],
                                None,
                                Rect::new(grid_x + (piece.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                                             grid_y + (piece.y + line_nb) as i32 * TETRIS_HEIGHT as i32,
                                         TETRIS_HEIGHT as u32, 
                                                TETRIS_HEIGHT as u32)
                                    ).expect("Couldn't copy texture into window");
                    }
                }
            }
        }

        if quit {  
            print_game_information(&tetris);          
            break
        }
       
        display_game_information(&tetris, &mut canvas, &texture_creator, &font, TETRIS_HEIGHT as i32 * 10 + 40);

        // 绘制地图中所有非0的格子，即已经合并过的，这里面没有正在移动的块，正在移动的块还没合并到地图里面
        for (line_nb, line) in tetris.game_map.iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
               if *case == 0 {
                  continue
               }
               canvas.copy(&textures[*case as usize - 1],
                  None,
                  Rect::new(grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                  grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                  TETRIS_HEIGHT as u32, TETRIS_HEIGHT as u32))
                  .expect("Couldn't copy texture into window");
            }
        }
               
        // 更新窗口显示
        canvas.present();

        // 每1秒60帧执行这个循环，所以要没1/60秒就sleep一下
        sleep(Duration::new(0, 1_000_000_000u32/60));
    }
}


