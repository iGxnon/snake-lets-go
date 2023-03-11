mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// 替换默认的 memory allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Represent a cell in snake-lets-go
/// each cell is represented as a single byte which
/// will map to a texture path
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Blank = 0,                  // blank 空白
    BodyClay = 1,               // clay 粘土
    BodyCoal = 2,               // coal_ore 煤矿
    BodyDiamond = 3,            // diamond 钻石矿
    BodyDirt = 4,               // dirt 泥土
    BodyEmerald = 5,            // emerald 绿宝石矿
    BodyGold = 6,               // gold 金矿
    BodyIron = 7,               // iron 铁矿
    BodyLapis = 8,              // lapis 青金石矿
    BodyStone = 9,              // stone 平滑石头
    BodyTNT = 10,               // TNT
    HeadDispenser = 11,         // dispenser 发射器 (正常表情)
    HeadDropper = 12,           // dropper 投掷器 (高兴)
    HeadObserver = 13,          // observer 观察者方块 (不高兴)
    SnackCookie = 14,           // cookie 曲奇
    SnackDriedKelp = 15,        // dried kelp 干海苔
    SnackPumpkinPie = 16,       // pumpkin pie 南瓜派
    SnackSpicyStrip = 17,       // rotten flesh 辣条 (腐肉)
    FruitGoldenApple = 18,      // apple golden 金苹果
    FoodBeef = 19,              // beef 牛肉
    FoodBread = 20,             // bread 面包
    FoodCake = 21,              // cake 蛋糕
    FoodChicken = 22,           // chicken 烧鸡
    FoodFish = 23,              // fish 煮鱼
    DrinkMilk = 24,             // milk 牛奶
    DrinkHoney = 25,            // honey 蜂蜜
    DrugLingeringHeal = 26,     // lingering heal 持续治疗
    DrugSplashHeal = 27,        // splash heal 瞬间治疗
    DrugSplashHealthBoost = 28, // splash health boost 生命提升
}
