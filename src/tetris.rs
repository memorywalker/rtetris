extern crate rand;

use std::time::SystemTime;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::score_file::print_game_information;

const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
const LEVEL_LINES: [u32; 10] = [5,   40,  60,  80,  100, 120, 140, 160, 180, 200];

type Piece = Vec<Vec<u8>>; // 表示一种二维图形
type States = Vec<Piece>;

pub struct Tetrimino {
    pub states: States,
    pub x: isize,  // 方块的坐标位置
    pub y: usize,
    pub current_state: u8, // 当前是哪一种状态，例如长条I有两种
}

impl Tetrimino {
    fn rotate(&mut self, game_map: &[Vec<u8>]) {
        // 旋转就认为时状态的变化
        let mut tmp_state = self.current_state + 1;
        // 状态不能超过最大情况
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }
        // 在水平方向尝试能不能找到合适的文位置，简化游戏
        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter() {
            if self.test_position(game_map, tmp_state as usize,
                                  self.x + x, self.y) == true {
                self.current_state = tmp_state; // 如果不冲突，就可以切换为这个形状
                self.x += *x;
                break
            }
        }
    }
    // 检测与网格中的其他元素是否冲突
    fn test_position(&self, game_map: &[Vec<u8>],
                     tmp_state: usize, x: isize, y: usize) -> bool {
        for shift_y in 0..4 {
            for shift_x in 0..4 {
                // 遍历方块当前状态的每一个点
                let x = x + shift_x;
                if self.states[tmp_state][shift_y][shift_x as usize] != 0 && // 方块中这个格子不为0
                    (y + shift_y >= game_map.len() || // y 方向没有超过网格的高度
                     x < 0 ||
                     x as usize >= game_map[y + shift_y].len() || // 没有超过行的最大宽度10
                     game_map[y + shift_y][x as usize] != 0) { // 和地图网格的当前位置的格子不冲突
                    return false;
                }
            }
        }
        return true;
    }
    
    fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }

    // 移动方块的位置
    fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
        if self.test_position(game_map, self.current_state as usize, new_x, new_y) == true {
            self.x = new_x as isize;
            self.y = new_y;
            true
        } else {
            false
        }
    }
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

struct TetriminoI;
// 数字1表示颜色
impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![1, 1, 1, 1],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoJ;

impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![2, 2, 2, 0],
                              vec![2, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![2, 2, 0, 0],
                              vec![0, 2, 0, 0],
                              vec![0, 2, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 0, 2, 0],
                              vec![2, 2, 2, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![2, 0, 0, 0],
                              vec![2, 0, 0, 0],
                              vec![2, 2, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoL;

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![3, 3, 3, 0],
                              vec![0, 0, 3, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 3, 0, 0],
                              vec![0, 3, 0, 0],
                              vec![3, 3, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![3, 0, 0, 0],
                              vec![3, 3, 3, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![3, 3, 0, 0],
                              vec![3, 0, 0, 0],
                              vec![3, 0, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoO;

impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![4, 4, 0, 0],
                              vec![4, 4, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 5,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoS;

impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![0, 5, 5, 0],
                              vec![5, 5, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 5, 0, 0],
                              vec![0, 5, 5, 0],
                              vec![0, 0, 5, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoZ;

impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![6, 6, 0, 0],
                              vec![0, 6, 6, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 0, 6, 0],
                              vec![0, 6, 6, 0],
                              vec![0, 6, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

struct TetriminoT;

impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![7, 7, 7, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 7, 0, 0],
                              vec![7, 7, 0, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 7, 0, 0],
                              vec![7, 7, 7, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 7, 0, 0],
                              vec![0, 7, 7, 0],
                              vec![0, 7, 0, 0],
                              vec![0, 0, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct Tetris {
    pub game_map: Vec<Vec<u8>>,// 16*10的网格
    pub current_level: u32,
    pub score: u32,
    pub nb_lines: u32, // 消除的总行数
    pub current_piece: Option<Tetrimino>,// 当前下落的方块
} 

impl Tetris {
    pub fn new() -> Tetris {
        // 地图大小为16行，每行10个格子
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        Tetris {
            game_map: game_map,
            current_level: 1, // 从1级开始
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }

    // 随机生成一个形状
    fn create_new_tetrimino(&self) -> Tetrimino {
        static mut PREV: u8 = 7; // 和C++中的静态变量作用相同
        let mut rand_nb = rand::random::<u8>() % 7;
        // 避免生成两个相同的，因为静态变量存在多线程同时访问的问题，所以是不安全的
        if unsafe { PREV } == rand_nb {
            rand_nb = rand::random::<u8>() % 7;
        }
        unsafe { PREV = rand_nb; }

        match rand_nb {
            0 => TetriminoI::new(),
            1 => TetriminoJ::new(),
            2 => TetriminoL::new(),
            3 => TetriminoO::new(),
            4 => TetriminoS::new(),
            5 => TetriminoZ::new(),
            6 => TetriminoT::new(),
            _ => unreachable!(),
        }
    }

    fn update_score(&mut self, to_add: u32) {
        self.score += to_add;
    }

    fn increase_level(&mut self) {
        self.current_level += 1;
    }
    // 消除的行数超过当前级别的行数要求后，级别增加一级
    fn increase_line(&mut self) {
        self.nb_lines += 1;
        if self.nb_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.increase_level();
        }
    }

    // 把一个块合并地图网格中
    fn make_permanent(&mut self) {
        let mut to_add = 0;
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;
            // 遍历当前块的y轴，并且当前位置的y不会超过地图的高度
            while shift_y < piece.states[piece.current_state as usize].len() &&
                  piece.y + shift_y < self.game_map.len() { 
                let mut shift_x = 0;
                // 遍历当前块的每一个x轴的格子不会超过地图的宽度
                while shift_x < piece.states[piece.current_state as usize][shift_y].len() &&
                      (piece.x + shift_x as isize) < self.game_map[piece.y + shift_y].len() as isize {
                    //如果块的当前格子不为0，需要把地图的这个格子也设置为块的格子的相同值，表示颜色
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] =
                            piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            // 合并一个块后增加分数
            to_add += self.current_level;
        }
        self.update_score(to_add);
        // 检查是否有可以删除的行
        self.check_lines();
        // 当前块已经被处理过了，所以设置为None
        self.current_piece = None;
    }

    fn check_lines(&mut self) {
        let mut remove_num = 0;
        let mut y = 0;
        let mut score_add = 0;
        // 遍历网格的每一行
        while y < self.game_map.len() {
            let mut complete = true;
            // 一行中有一个格子是0，说明不能消除
            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break
                }
            }
            // 如果这一行可以消除
            if complete == true {
                score_add += self.current_level;
                self.game_map.remove(y);
                remove_num += 1;
                y -= 1;
            }
            y += 1;
        }
        // 连消4行
        if remove_num == 4 {
            // A "tetris"!
            score_add += 1000;
        }
        self.update_score(score_add);
        while self.game_map.len() < 16 {
            self.increase_line();
            // 补上消除的行，保证网格还是16*10
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }    
}

pub fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime,
                event_pump: &mut sdl2::EventPump) -> bool {
    // 一个块正在下落
    let mut make_permanent = false;
    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    *quit = true;
                    break
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    *timer = SystemTime::now();// 更新下落的计时器
                    tmp_y += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    tmp_x += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    tmp_x -= 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    let x = piece.x;
                    let mut y = piece.y;
                    // 手动快速下降到底部或有冲突不能移动
                    while piece.change_position(&tetris.game_map, x, y + 1) == true {
                        y += 1;
                    }
                    // 不能移动了，所以标记为需要合并到网格地图
                    make_permanent = true;
                }
                _ => {}
            }
        }
        // 根据按键后的坐标位置移动方块
        if !make_permanent {
            // 如果不能移动，且当前y的值也没有变化，说明已经移动到最下面了，需要合并方块到网格
            if piece.change_position(&tetris.game_map, tmp_x, tmp_y) == false && tmp_y != piece.y {
                make_permanent = true;
            }
        }
    }
    if make_permanent {
        // 合并方块后，更新计时器
        tetris.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}

// 判断是否需要处理下落的时间到了
fn is_time_over(tetris: &Tetris, timer: &SystemTime) -> bool {
    match timer.elapsed() {
        Ok(elapsed) => {
            // 得到毫秒值
            let millis = elapsed.as_secs() as u32 * 1000 + elapsed.subsec_nanos() / 1_000_000;
            millis > LEVEL_TIMES[tetris.current_level as usize - 1]
        }
        Err(_) => false,
    }
}

pub fn falling(tetris: & mut Tetris, timer: &mut SystemTime) {
    if is_time_over(&tetris, &timer) {
        let mut make_permanent = false;
        if let Some(ref mut piece) = tetris.current_piece {
          let x = piece.x;
          let y = piece.y + 1;
          make_permanent = !piece.change_position(&tetris.game_map,
             x, y);
        }
        if make_permanent {
          tetris.make_permanent();
        }        
        *timer = SystemTime::now();
    }
}

pub fn update_tetris(tetris: & mut Tetris) -> bool {
    let mut ret = true;
    if tetris.current_piece.is_none() {
        let current_piece = tetris.create_new_tetrimino();
        if !current_piece.test_current_position(&tetris.game_map) {     
            print_game_information(tetris);       
            ret = false; // 新创建的方块就已经冲突了，说明游戏结束了
        } else {
            tetris.current_piece = Some(current_piece);
            ret = true;
        }       
    }
    ret
}