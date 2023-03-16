use crate::utils::Timer;
use chrono::Local;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Represent a cell in snake-lets-go
/// each cell is represented as a single byte which
/// will map to a texture path
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Blank = 0, // blank 空白
    BodyClay = 1,
    // clay 粘土
    BodyCoal = 2,
    // coal_ore 煤矿
    BodyDiamond = 3,
    // diamond 钻石矿
    BodyDirt = 4,
    // dirt 泥土
    BodyEmerald = 5,
    // emerald 绿宝石矿
    BodyGold = 6,
    // gold 金矿
    BodyIron = 7,
    // iron 铁矿
    BodyLapis = 8,
    // lapis 青金石矿
    BodyStone = 9,
    // stone 平滑石头
    BodyTNT = 10,
    // TNT
    HeadDispenser = 11,
    // dispenser 发射器 (正常表情)
    HeadDropper = 12,
    // dropper 投掷器 (高兴)
    HeadObserver = 13,
    // observer 观察者方块 (不高兴)
    SnackCookie = 14,
    // cookie 曲奇
    SnackDriedKelp = 15,
    // dried kelp 干海苔
    SnackPumpkinPie = 16,
    // pumpkin pie 南瓜派
    SnackSpicyStrip = 17,
    // rotten flesh 辣条 (腐肉)
    FruitGoldenApple = 18,
    // apple golden 金苹果
    FoodBeef = 19,
    // beef 牛肉
    FoodBread = 20,
    // bread 面包
    FoodCake = 21,
    // cake 蛋糕
    FoodChicken = 22,
    // chicken 烧鸡
    FoodFish = 23,
    // fish 煮鱼
    DrinkMilk = 24,
    // milk 牛奶
    DrinkHoney = 25,
    // honey 蜂蜜
    DrugLingeringHeal = 26,
    // lingering heal 持续治疗
    DrugSplashHeal = 27,
    // splash heal 瞬间治疗
    DrugSplashHealthBoost = 28, // splash health boost 生命提升
}

impl Cell {
    #[inline]
    fn blank(&mut self) {
        *self = Cell::Blank;
    }

    #[inline]
    fn eatable(&mut self) {
        let prob: u8 = rand::random();
        match prob % 5 {
            0 => self.drug(),
            1 => self.drink(),
            2 => self.food(),
            3 => self.fruit(),
            4 => self.snack(),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn is_eatable(&self) -> bool {
        *self as u8 >= 14
    }

    // Safety: cast 26-28 to Cell
    #[inline]
    fn drug(&mut self) {
        let prob: u8 = rand::random();
        unsafe {
            *self = core::mem::transmute((prob % 3) + 26);
        }
    }

    #[inline]
    fn is_drug(&self) -> bool {
        let id = *self as u8;
        (26..=28).contains(&id)
    }

    #[inline]
    fn drink(&mut self) {
        if rand::random() {
            *self = Cell::DrinkMilk;
        } else {
            *self = Cell::DrinkHoney;
        }
    }

    #[inline]
    fn is_drink(&self) -> bool {
        *self == Cell::DrinkHoney || *self == Cell::DrinkMilk
    }

    // Safety: cast 19-23 to Cell
    #[inline]
    fn food(&mut self) {
        let prob: u8 = rand::random();
        unsafe {
            *self = core::mem::transmute((prob % 5) + 19);
        }
    }

    #[inline]
    fn is_food(&self) -> bool {
        let id = *self as u8;
        (19..=23).contains(&id)
    }

    #[inline]
    fn fruit(&mut self) {
        *self = Cell::FruitGoldenApple;
    }

    #[inline]
    fn is_fruit(&self) -> bool {
        *self == Cell::FruitGoldenApple
    }

    // Safety: cast 14-17 to Cell
    #[inline]
    fn snack(&mut self) {
        let prob: u8 = rand::random();
        unsafe {
            *self = core::mem::transmute((prob % 4) + 14);
        }
    }

    #[inline]
    fn is_snack(&self) -> bool {
        let id = *self as u8;
        (14..=17).contains(&id)
    }

    #[inline]
    fn head_normal(&mut self) {
        *self = Cell::HeadDispenser;
    }

    #[inline]
    fn head_happy(&mut self) {
        *self = Cell::HeadDropper;
    }

    #[inline]
    fn head_unhappy(&mut self) {
        *self = Cell::HeadObserver;
    }

    #[inline]
    fn is_head(&self) -> bool {
        *self == Cell::HeadDispenser || *self == Cell::HeadObserver || *self == Cell::HeadDropper
    }

    // Safety: cast 1-10 to Cell
    #[inline]
    fn body(&mut self) {
        let prob: u8 = rand::random();
        unsafe {
            *self = core::mem::transmute((prob % 10) + 1);
        }
    }

    #[inline]
    fn is_body(&self) -> bool {
        let id = *self as u8;
        (1..=10).contains(&id)
    }
}

// just a status container
pub struct Snake {
    head: (i32, i32),                 // (row, col)
    directions: VecDeque<(i32, i32)>, // [(d_row, d_col)]
    tail: (i32, i32),                 // (row, col, d_row, d_col)
    goods: u32,                       // 连续吃正向食物
    snacks: u32,                      // 连续吃零食
    drugs: u8,                        // 连续吃药
    hunger: u32,                      // 连续挨饿
    hunger_cnt: u8,                   // 饥饿 buff2 下吃正向食物计数 (不一定需要连续)
    speed: u8,                        // 速度
    // 位: 76543210
    // 0: 是否处于饥饿状态(生长缓慢buff2)
    // 1: 是否处于头晕状态
    // 3~7: 保留
    timing_buff: u8, // buff
    length: u32,
}

const HUNGRY_BUFF: u8 = 0;
const DIZZINESS_BUFF: u8 = 1;

pub struct Refreshes {
    // 上次食物刷新时间 (ms)
    last_eatable_refresh: i64,
    last_eatable_pos: Vec<usize>,
    // 上次吃东西时间 (ms)
    last_feed: i64,
    // 上次创墙时间 (ms)
    last_knock_wall: i64,
    // 上次移动时间 (ms)
    last_move: i64,
}

enum Status {
    Pause,
    Start,
}

#[wasm_bindgen]
pub struct Game {
    size: usize,
    input_directions: (i32, i32),
    snake: Snake,
    refreshes: Refreshes,
    status: Status,
    cells: Vec<Cell>,
}

#[inline]
fn get_index(size: usize, row: usize, column: usize) -> usize {
    row * size + column
}

#[wasm_bindgen]
impl Game {
    pub fn new_with_size(size: usize) -> Game {
        utils::set_panic_hook();
        let mut cells: Vec<_> = (0..size * size).map(|_| Cell::Blank).collect();
        let head_row = size >> 2;
        let head_col = 2;
        let now = Local::now().timestamp_millis();
        cells[get_index(size, head_row, head_col)].head_normal();
        cells[get_index(size, head_row, head_col - 1)].body();
        Game {
            size,
            cells,
            snake: Snake {
                head: (head_row as i32, head_col as i32),
                directions: vec![(0, 1), (0, 1)].into(),
                tail: (head_row as i32, (head_col - 1) as i32),
                goods: 0,
                snacks: 0,
                drugs: 0,
                hunger: 0,
                hunger_cnt: 0,
                speed: 3,
                timing_buff: 0,
                length: 2,
            },
            refreshes: Refreshes {
                last_eatable_refresh: 0,
                last_eatable_pos: Vec::new(),
                last_feed: now,
                last_knock_wall: 0,
                last_move: now,
            },
            status: Status::Pause,
            input_directions: (0, 1),
        }
    }

    #[inline]
    fn get_index(&self, row: usize, column: usize) -> usize {
        get_index(self.size, row, column)
    }

    #[inline]
    fn try_refresh_food(&mut self, ts: i64) -> bool {
        debug_assert!(ts > self.refreshes.last_eatable_refresh);
        debug_assert!(ts > self.refreshes.last_feed);
        if ts - self.refreshes.last_eatable_refresh > 5_000 {
            // 移除上一次的食物
            for &pos in &self.refreshes.last_eatable_pos {
                if self.cells[pos].is_eatable() {
                    // 判断一下还是不是食物，吃掉的就不删了
                    self.cells[pos].blank();
                }
            }

            // 刷新下一次的食物
            self.refreshes.last_eatable_pos = Vec::new();
            let eatable_num: u8 = rand::random();
            while self.refreshes.last_eatable_pos.len() < ((eatable_num % 3) + 3) as usize {
                let pos: usize = rand::random();
                let pos = pos % (self.size * self.size);
                if self.cells[pos] == Cell::Blank {
                    self.cells[pos].eatable();
                    self.refreshes.last_eatable_pos.push(pos);
                }
            }

            // 食物刷新时判断一下 进食时间是否超过 5s
            if ts - self.refreshes.last_feed > 5_000 {
                // 连续饥饿 +1
                self.snake.hunger += 1;
            } else {
                // 连续饥饿清零
                self.snake.hunger = 0;
            }
            if self.snake.hunger == 2 {
                // 连续 2 次饥饿，加上饥饿 buff
                self.snake.timing_buff |= 1 << HUNGRY_BUFF;
            }
            if self.snake.hunger == 3 {
                // 连续 3 次饥饿，game over
                return false;
            }

            self.refreshes.last_eatable_refresh = ts;
        }
        true
    }

    #[inline]
    fn check_buff(&mut self, ts: i64) {
        debug_assert!(ts > self.refreshes.last_knock_wall);
        if ts - self.refreshes.last_knock_wall > 20_000 {
            // 超过 20s 取消 buff
            self.clear_dizziness_buff();
        }
    }

    #[inline]
    fn clear_dizziness_buff(&mut self) {
        if self.snake.timing_buff & (1 << DIZZINESS_BUFF) > 0 {
            self.snake.speed *= 3;
            self.snake.timing_buff &= !(1 << DIZZINESS_BUFF);
        }
    }

    #[inline]
    fn clear_hunger_buff(&mut self) {
        if self.snake.timing_buff & (1 << HUNGRY_BUFF) > 0 {
            self.snake.timing_buff &= !(1 << HUNGRY_BUFF);
        }
    }

    // try_move: 这一部分比较难理解，我尽可能地加上注释
    #[inline]
    fn try_move(&mut self, ts: i64) -> bool {
        debug_assert!(ts > self.refreshes.last_move);
        debug_assert!(ts > self.refreshes.last_knock_wall);
        let wait_ts = 800 / (self.snake.speed as i64);
        if ts - self.refreshes.last_move > wait_ts {
            self.refreshes.last_move = ts;

            let head = self.snake.head;
            let head_direction = self.snake.directions[0];
            let next_head = (head.0 + head_direction.0, head.1 + head_direction.1);

            // 更新新的头部方向 (下一个头部的方向)
            let mut next_direction = if self.snake.timing_buff & (1 << DIZZINESS_BUFF) > 0
                && (head_direction.0 != self.input_directions.0
                    || head_direction.1 != self.input_directions.1)
            {
                // 如果当前方向改变了 (input_directions不是之前的head_direction)
                // 并且处于 DIZZINESS_BUFF 状态，下一次的输入方向反转
                (-self.input_directions.0, -self.input_directions.1)
            } else {
                (self.input_directions.0, self.input_directions.1)
            };

            // 判断输入方向是否合理(掉头跑)，否则强行修改成当前头部的方向
            if next_direction.0 == 0
                && head_direction.0 == 0
                && next_direction.1 == -head_direction.1
            {
                next_direction = head_direction;
            }
            if next_direction.1 == 0
                && head_direction.1 == 0
                && next_direction.0 == -head_direction.0
            {
                next_direction = head_direction;
            }

            // 当前头部坐标
            let current = self.get_index(head.0 as usize, head.1 as usize);

            // 判断下次头部坐标是否撞墙，以及连续撞墙判定
            if next_head.0 >= self.size as i32
                || next_head.1 >= self.size as i32
                || next_head.0 < 0
                || next_head.1 < 0
            {
                return if ts - self.refreshes.last_knock_wall < 3_000 {
                    // 连续撞墙，game over
                    false
                } else {
                    if self.snake.timing_buff & 1 << DIZZINESS_BUFF == 0 {
                        // 如果之前没有头晕buff，或者头晕 buff 过期了，撞墙后速度再变慢
                        // 防止多次撞墙减速后速度成 0，然后上面出现 divide by zero
                        self.snake.speed /= 3;
                    }
                    self.snake.timing_buff |= 1 << DIZZINESS_BUFF; // 加上头晕 buff
                    self.refreshes.last_knock_wall = ts; // 更新撞墙时间
                    self.cells[current].head_unhappy(); // 改成不开心表情
                    self.snake.directions.pop_front(); // 删掉导致撞墙的方向(这个方向没有实际上使用到，所以要删掉，防止传播到尾部导致 bug)
                    self.snake.directions.push_front(next_direction); // 补上下个输入方向当作当前头部的修正方向
                    true
                };
            }

            // 添上下一次头部的方向到队列中
            self.snake.directions.push_front(next_direction);
            // 更新头部坐标
            self.snake.head = next_head;
            // 将当前头部的地方改成身体 PS: 随机改变身体，可能会变成彩虹蛇(
            self.cells[current].body();

            // 下一个头部的坐标
            let next = self.get_index(next_head.0 as usize, next_head.1 as usize);

            if self.cells[next].is_body() {
                // 吃到身体了
                return false;
            }

            if self.cells[next].is_eatable() {
                // 吃到了可以吃的东西

                // 修改连续计数器
                if self.cells[next].is_snack() {
                    self.snake.snacks += 1;
                    self.snake.goods = 0;
                    self.snake.drugs = 0;
                } else if self.cells[next].is_drug() {
                    self.snake.drugs += 1;
                    self.snake.goods = 0;
                    self.snake.snacks = 0;
                } else {
                    self.snake.goods += 1;
                    self.snake.hunger_cnt += 1;
                    self.snake.snacks = 0;
                    self.snake.drugs = 0;
                }

                if self.snake.goods >= 4 {
                    // 连续 4 次正向食物，清除 hunger buff
                    self.clear_hunger_buff();
                }

                // 计算速度翻倍
                let mut speed_up = self.snake.goods / 6;
                while speed_up > 0 {
                    // (实际测试下来速度翻倍会变得很难)
                    self.snake.speed *= 2;
                    speed_up -= 1;
                    self.snake.goods = 0;
                }

                if self.snake.snacks >= 3 {
                    // 连续吃 3 次零食，速度恢复正常
                    self.snake.speed = 3;
                }

                if self.snake.drugs >= 2 {
                    // 连续吃药 game over;
                    return false;
                }

                // if 写开一点方便查看
                if !self.cells[next].is_drug() {
                    // 吃到的不是药品
                    if self.snake.timing_buff & (1 << HUNGRY_BUFF) == 0 {
                        // 没有饥饿 buff2
                        if self.snake.snacks == 0 {
                            // 吃到了正向食物
                            // 这时需要变长
                            self.snake.length += 1;
                            self.refreshes.last_feed = ts;
                            // 吃到了金苹果！！
                            if self.cells[next] == Cell::FruitGoldenApple {
                                self.cells[next].head_happy();
                            } else {
                                self.cells[next].head_normal();
                            }
                            return true;
                        }
                        // 吃了零食，少长一半！
                        if self.snake.snacks % 2 != 0 {
                            self.snake.length += 1;
                            self.refreshes.last_feed = ts;
                            self.cells[next].head_normal();
                            return true;
                        }
                    }
                    if self.snake.hunger_cnt >= 2 {
                        // 有饥饿 buff2，但连续吃了两次正向食物
                        // 这时需要变长
                        self.snake.length += 1;
                        self.snake.hunger_cnt = 0;
                        self.refreshes.last_feed = ts;
                        self.cells[next].head_normal();
                        return true;
                    }
                    // 饥饿 buff2 下吃到了零食不变长
                    self.refreshes.last_feed = ts; // TODO: 是否该刷新 last_feed ?
                } else {
                    // 清除负面 buff
                    self.clear_dizziness_buff();
                    self.clear_hunger_buff();
                    self.snake.snacks = 0;
                }
            }

            // 将下一个头部方向的方块改成头部
            self.cells[next].head_normal();

            let tail = self.snake.tail;
            // 移除最后一个尾巴的方向
            let tail_direction = self.snake.directions.pop_back().unwrap();
            // 下一个尾巴的坐标
            let next_tail = (tail.0 + tail_direction.0, tail.1 + tail_direction.1);
            // 当前尾部坐标
            let current = self.get_index(tail.0 as usize, tail.1 as usize);
            // 移除尾巴
            self.cells[current].blank();
            // 更新尾巴坐标
            self.snake.tail = next_tail;
        }
        true
    }

    pub fn input(&mut self, d_row: i32, d_col: i32) {
        self.input_directions.0 = d_row;
        self.input_directions.1 = d_col;
    }

    pub fn start(&mut self) {
        self.status = Status::Start;
    }

    pub fn pause(&mut self) {
        self.status = Status::Pause;
    }

    pub fn tick(&mut self) -> bool {
        if matches!(self.status, Status::Pause) {
            return true;
        }
        let _timer = Timer::new("Game::tick"); // profiler
        let ts = Local::now().timestamp_millis();
        self.check_buff(ts);
        if !self.try_refresh_food(ts) {
            return false;
        }
        if !self.try_move(ts) {
            return false;
        }
        true
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn buffs(&self) -> u8 {
        self.snake.timing_buff
    }

    pub fn hungers(&self) -> u32 {
        self.snake.hunger
    }

    pub fn length(&self) -> u32 {
        self.snake.length
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.size) {
            for &cell in line {
                write!(f, "{:3}", cell as u8)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_cell() {
        let mut cell = Cell::Blank;
        cell.food();
        assert!(cell as u8 >= 19 && cell as u8 <= 23);
        cell.drug();
        assert!(cell as u8 >= 26 && cell as u8 <= 28);
        cell.fruit();
        assert!(matches!(cell, Cell::FruitGoldenApple));
        cell.drink();
        assert!(cell as u8 >= 24 && cell as u8 <= 25);
    }

    #[test]
    fn test_game() {
        let game = Game::new_with_size(18);
        println!("{}", game);
    }
}
