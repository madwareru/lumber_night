use std::cmp::Ordering;
use std::collections::VecDeque;
use crate::objects::{Object};
use crate::random::RndGen;
use crate::tilemap::Tilemap;
use super::wasm4::*;
use bitsetium::{BitSearch, BitEmpty, BitSet, BitIntersection, BitUnion, BitTestNone, BitTest};

type PassabilitySet = [u8; 200];

const OBJECTS: [(Object, (i16, i16));114] = [
    (Object::Pine, (12,4)),
    (Object::LittlePine, (20,4)),
    (Object::LittlePine, (144,4)),
    (Object::Oak, (156,4)),
    (Object::Pine, (16,8)),
    (Object::Oak, (24,8)),
    (Object::LittlePine, (100,8)),
    (Object::Pine, (108,8)),
    (Object::LittlePine, (20,12)),
    (Object::BigHouse, (64,12)),
    (Object::LittlePine, (96,12)),
    (Object::LittleOak, (104,12)),
    (Object::LittlePine, (120,12)),
    (Object::LittlePine, (128,12)),
    (Object::LittlePine, (100,16)),
    (Object::LittleOak, (124,16)),
    (Object::LittlePine, (120,20)),
    (Object::Pine, (128,20)),
    (Object::LittlePine, (116,24)),
    (Object::Pine, (124,24)),
    (Object::Oak, (136,24)),
    (Object::Oak, (144,24)),
    (Object::Pine, (120,28)),
    (Object::Pine, (140,28)),
    (Object::Pine, (20,32)),
    (Object::Oak, (32,32)),
    (Object::Oak, (132,32)),
    (Object::Oak, (24,36)),
    (Object::Worker, (60,40)),
    (Object::Worker, (80,40)),
    (Object::LittleOak, (120,40)),
    (Object::Worker, (68,44)),
    (Object::Oak, (24,56)),
    (Object::LittleOak, (16,60)),
    (Object::Oak, (4,64)),
    (Object::LittleOak, (12,64)),
    (Object::Pine, (8,68)),
    (Object::LittlePine, (96,68)),
    (Object::LittleOak, (100,92)),
    (Object::LittlePine, (48,96)),
    (Object::LittlePine, (52,100)),
    (Object::LittleOak, (76,100)),
    (Object::LittlePine, (112,104)),
    (Object::Oak, (148,104)),
    (Object::LittlePine, (156,104)),
    (Object::Oak, (152,108)),
    (Object::LittleOak, (140,112)),
    (Object::Oak, (32,120)),
    (Object::LittleOak, (38,120)),
    (Object::LittlePine, (80,120)),
    (Object::Pine, (88,120)),
    (Object::LittlePine, (152,120)),
    (Object::LittleOak, (36,124)),
    (Object::Oak, (84,124)),
    (Object::LittlePine, (112,124)),
    (Object::LittleOak, (28,128)),
    (Object::LittlePine, (104,128)),
    (Object::LittlePine, (156,128)),
    (Object::LittleOak, (24,132)),
    (Object::Oak, (64,132)),
    (Object::LittlePine, (72,132)),
    (Object::LittlePine, (4,136)),
    (Object::Oak, (12,136)),
    (Object::LittleOak, (32,136)),
    (Object::LittleOak, (60,136)),
    (Object::Oak, (68,136)),
    (Object::Pine, (76,136)),
    (Object::LittlePine, (120,136)),
    (Object::LittlePine, (8,140)),
    (Object::LittlePine, (20,140)),
    (Object::LittleOak, (64,140)),
    (Object::LittlePine, (72,140)),
    (Object::LittleOak, (84,140)),
    (Object::LittleOak, (128,140)),
    (Object::Oak, (156,140)),
    (Object::LittlePine, (0,144)),
    (Object::Oak, (16,144)),
    (Object::LittlePine, (80,144)),
    (Object::LittleOak, (88,144)),
    (Object::Oak, (116,144)),
    (Object::Oak, (148,144)),
    (Object::Oak, (4,148)),
    (Object::Pine, (12,148)),
    (Object::LittleOak, (20,148)),
    (Object::Oak, (84,148)),
    (Object::LittleOak, (140,148)),
    (Object::Pine, (156,148)),
    (Object::Oak, (8,152)),
    (Object::LittleOak, (16,152)),
    (Object::Pine, (24,152)),
    (Object::LittlePine, (32,152)),
    (Object::LittleOak, (40,152)),
    (Object::Oak, (48,152)),
    (Object::LittlePine, (92,152)),
    (Object::Pine, (136,152)),
    (Object::LittlePine, (144,152)),
    (Object::LittlePine, (152,152)),
    (Object::Oak, (4,156)),
    (Object::LittlePine, (12,156)),
    (Object::Oak, (20,156)),
    (Object::Pine, (28,156)),
    (Object::Pine, (36,156)),
    (Object::LittlePine, (44,156)),
    (Object::Oak, (52,156)),
    (Object::LittlePine, (72,156)),
    (Object::Pine, (80,156)),
    (Object::Pine, (88,156)),
    (Object::Pine, (96,156)),
    (Object::Oak, (112,156)),
    (Object::Pine, (120,156)),
    (Object::LittlePine, (132,156)),
    (Object::LittlePine, (140,156)),
    (Object::Pine, (148,156)),
    (Object::LittleOak, (156,156)),
];

macro_rules! with_taken {
    ($ex:expr, as $i:ident => $body:tt) => {
        let mut $i = std::mem::take(&mut $ex);
        {
            $body
        }
        $ex = $i;
    }
}

#[derive(Copy, Clone)]
struct HealthPoints {
    amount: u8
}

#[derive(Copy, Clone)]
struct AgePoints {
    amount: u16
}

#[derive(Copy, Clone)]
struct Bag {
    count_items: u8,
    max_items: u8
}

#[derive(Copy, Clone)]
struct StoreHouse {
    count_items: u64
}

#[derive(Copy, Clone)]
struct RippleOffset(i8, i8);

const GROW_TICKS: usize = 12;
const WATER_ANIMATION_TICKS: usize = 10;
const OAK_AGE_TILL_MATURE: u16 = 100;
const PINE_AGE_TILL_MATURE: u16 = 20;
const PINE_HP_MAX: u8 = 15;
const OAK_HP_MAX: u8 = 21;

struct EcsWorld {
    pub workers_archetype: Vec<((i16, i16), Bag)>,
    pub ripples_archetype: Vec<((i16, i16), RippleOffset)>,
    pub store_house_archetype: Vec<((i16, i16), StoreHouse)>,
    pub mature_tree_archetype: Vec<(Object, (i16, i16), HealthPoints )>,
    pub young_tree_archetype: Vec<(Object, (i16, i16), AgePoints )>,
    pub dead_tree_archetype: Vec<((i16, i16), )>,
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum Direction { None, Up, Down, Left, Right }

pub struct GameStage {
    rnd_gen: RndGen,
    obstacles: PassabilitySet,
    visited_fields: PassabilitySet,
    path_buffer: [Direction; 40*40],
    path_queue: VecDeque<((u8, u8), Direction)>,
    mouse_path: Vec<Direction>,
    current_frame: usize,
    ecs_world: EcsWorld,
    tilemap: Tilemap,
    render_buffer: Vec<(Object, (i16, i16))>
}

impl GameStage {
    pub fn new() -> Self {
        GameStage {
            rnd_gen: RndGen::new(),
            obstacles: PassabilitySet::empty(),
            visited_fields: PassabilitySet::empty(),
            path_buffer: [Direction::None; 40*40],
            path_queue: VecDeque::with_capacity(1024),
            mouse_path: Vec::new(),
            ecs_world: EcsWorld {
                workers_archetype: Vec::new(),
                ripples_archetype: Vec::new(),
                store_house_archetype: Vec::new(),
                mature_tree_archetype: Vec::new(),
                young_tree_archetype: Vec::new(),
                dead_tree_archetype: Vec::new()
            },
            current_frame: 0,
            tilemap: Tilemap {
                map: [
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                    0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
                    0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0,
                    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0,
                    0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,
                    0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1,
                    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ]
            },
            render_buffer: Vec::with_capacity(200)
        }
    }

    pub fn start(&mut self) {
        unsafe {
            *PALETTE = [0x232e45, 0x3c5d75, 0x5eb2a0, 0xffd7b9];
        }

        let mut last_set = false;
        for j in 0..40 {
            for i in 0..40 {
                let idx = ((j / 2) * 21 + i / 2) as usize;

                let mut obstacle_offset = 0;
                if j % 2 != 0 { obstacle_offset += 21; }
                if i % 2 != 0 { obstacle_offset += 1; }

                if self.tilemap.map[idx + obstacle_offset] == 0 {
                    self.obstacles.set(j as usize * 40 + i as usize);
                } else {
                    continue;
                }

                // if self.tilemap.map[idx] == 1 &&
                //     self.tilemap.map[idx + 1] == 1 &&
                //     self.tilemap.map[idx + 21] == 1 &&
                //     self.tilemap.map[idx + 22] == 1 {
                //     continue;
                // }

                if last_set {
                    last_set = false;
                    continue;
                }

                if self.rnd_gen.gen_range(0..=99) < 30 {
                    let x = i * 4 + self.rnd_gen.gen_range_i(-2..=2);
                    let y = j * 4 + self.rnd_gen.gen_range_i(-1..=1);
                    let (off_x, off_y) = (
                        self.rnd_gen.gen_range_i(-1..=1) as i8,
                        self.rnd_gen.gen_range_i(-1..=1) as i8
                    );
                    self.ecs_world.ripples_archetype.push(((x, y), RippleOffset(off_x, off_y)));
                    last_set = true;
                }
            }
        }


        for &(object, (x, y)) in OBJECTS.iter() {
            match object {
                Object::BigHouse => {
                    let yy = (y / 4) as usize;
                    let xx = (x / 4) as usize;
                    for j in yy - 2 .. yy + 4 {
                        for i in xx - 2 .. xx + 4 {
                            let passability_tile_idx = j * 40 + i;
                            self.obstacles.set(passability_tile_idx);
                        }
                    }
                    self.ecs_world.store_house_archetype.push(
                        ((x, y), StoreHouse { count_items: 0 })
                    );
                }
                Object::Worker => {
                    self.ecs_world.workers_archetype.push(
                        (
                            (x, y),
                            Bag {
                                count_items: 0,
                                max_items: 3
                            }
                        )
                    );
                }
                _ => {
                    let passability_tile_idx = ((y / 4) as usize * 40 + (x / 4) as usize);
                    self.obstacles.set(passability_tile_idx);

                    let (hp, is_mature) = match object {
                        Object::Pine => (PINE_HP_MAX, true),
                        Object::Oak => (OAK_HP_MAX, true),
                        Object::LittlePine => (PINE_HP_MAX, false),
                        Object::LittleOak => (OAK_HP_MAX, false),
                        _ => (0, false)
                    };
                    if hp > 0 {
                        if is_mature {
                            self.ecs_world.mature_tree_archetype.push(
                                (
                                    object,
                                    (x, y),
                                    HealthPoints { amount: hp }
                                )
                            );
                        } else {
                            self.ecs_world.young_tree_archetype.push(
                                (
                                    object,
                                    (x, y),
                                    AgePoints { amount: 0 }
                                )
                            )
                        }
                    } else {
                        self.ecs_world.dead_tree_archetype.push(
                            (
                                (x, y),
                            )
                        );
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        if self.current_frame % WATER_ANIMATION_TICKS == 0 {
            self.update_ripples();
        }
        if self.current_frame % GROW_TICKS == 0 {
            self.grow_trees();
        }
        self.update_paths();
        self.current_frame += 1;
    }

    fn grow_trees(&mut self) {
        let mut rng = std::mem::take(&mut self.rnd_gen);
        // make some living for long dead trees:
        {
            for stump_i in (0..self.ecs_world.dead_tree_archetype.len()).rev() {
                if rng.gen_range(0..=1000) < 2 {
                    let ((x, y),) = self.ecs_world.dead_tree_archetype.swap_remove(stump_i);
                    self.ecs_world.young_tree_archetype.push(
                        (
                            if rng.gen_range(0..=9) >= 7 { Object::Oak } else { Object::Pine },
                            (x, y),
                            AgePoints { amount: 0 }
                        )
                    )
                }
            }
        }

        // grow youngs with a sparkle of random
        {
            // find grown ups and just move them to the matures
            {
                for young_i in (0..self.ecs_world.young_tree_archetype.len()).rev() {
                    let (object, position, age_pts) = self.ecs_world.young_tree_archetype[young_i];
                    match (object, age_pts) {
                        (Object::LittleOak, age_points) if age_points.amount >= OAK_AGE_TILL_MATURE => {
                            self.ecs_world.young_tree_archetype.swap_remove(young_i);
                            self.ecs_world.mature_tree_archetype.push(
                                (
                                    Object::Oak,
                                    position,
                                    HealthPoints { amount: OAK_HP_MAX }
                                )
                            );
                        },
                        (Object::LittlePine, age_points) if age_points.amount >= PINE_AGE_TILL_MATURE => {
                            self.ecs_world.young_tree_archetype.swap_remove(young_i);
                            self.ecs_world.mature_tree_archetype.push(
                                (
                                    Object::Pine,
                                    position,
                                    HealthPoints { amount: PINE_HP_MAX }
                                )
                            )
                        },
                        _ => ()
                    }
                }
            }

            // iterate through the rest of youngs and grow them
            {
                for (obj, _, age_pts ) in self.ecs_world.young_tree_archetype.iter_mut() {
                    let is_grow = match obj {
                        Object::LittlePine => {
                            rng.gen_range(0..=99) < 45 // probability of pine growth is decently high
                        },
                        Object::LittleOak => {
                            rng.gen_range(0..=99) < 30 // oaks grow slower
                        },
                        _ => false
                    };

                    if !is_grow { continue; }

                    age_pts.amount += 1;
                }
            }
        }
        self.rnd_gen = rng;
    }

    fn update_ripples(&mut self) {
        let mut rng = std::mem::take(&mut self.rnd_gen);
        for ripple in self.ecs_world.ripples_archetype.iter_mut() {
            if rng.gen_range(0..=99) > 40 {
                continue;
            }
            let RippleOffset(_, off_y) = ripple.1;
            let off_x = rng.gen_range_i(-1..=1) as i8;
            ripple.1 = RippleOffset(off_x, off_y);
        }
        self.rnd_gen = rng;
    }

    fn update_paths(&mut self) {
        let (mx, my) = unsafe{
            ((*super::wasm4::MOUSE_X) / 4, (*super::wasm4::MOUSE_Y) / 4)
        };
        if !((0..40).contains(&mx) && (0..40).contains(&my)) {
            return;
        }
        let mut pvec = std::mem::take(&mut self.mouse_path);
        pvec.clear();
        self.find_path((17, 11), (mx as u8, my as u8), &mut pvec);
        self.mouse_path = pvec;
    }

    pub fn render(&mut self) {
        self.render_ripples();
        self.tilemap.draw();
        self.render_objects();

        if !self.mouse_path.is_empty() {
            unsafe {*super::wasm4::DRAW_COLORS = 4;}
            let (mut cx, mut cy) = (68, 44);
            for dir in self.mouse_path.iter() {
                match dir {
                    Direction::None => {}
                    Direction::Up => {
                        super::wasm4::line(cx, cy, cx, cy - 4);
                        cy -= 4;
                    }
                    Direction::Down => {
                        super::wasm4::line(cx, cy, cx, cy + 4);
                        cy += 4;
                    }
                    Direction::Left => {
                        super::wasm4::line(cx - 4, cy, cx, cy);
                        cx -= 4;
                    }
                    Direction::Right => {
                        super::wasm4::line(cx + 4, cy, cx, cy);
                        cx += 4;
                    }
                }
            }
        }
    }

    fn render_ripples(&self) {
        for ((x, y), RippleOffset(mut off_x, mut off_y)) in self.ecs_world.ripples_archetype.iter() {
            let x = *x as i32 + off_x as i32;
            let y = *y as i32 + off_y as i32;
            unsafe {*super::wasm4::DRAW_COLORS = 2;}
            super::wasm4::line(x, y, x + 3, y);
        }
    }

    fn render_objects(&mut self) {
        self.render_buffer.extend(
            self.ecs_world.workers_archetype.iter().map(|it|
                (Object::Worker, it.0)
            )
        );
        self.render_buffer.extend(
            self.ecs_world.mature_tree_archetype.iter().map(|it|
                (it.0, it.1)
            )
        );
        self.render_buffer.extend(
            self.ecs_world.young_tree_archetype.iter().map(|it|
                (it.0, it.1)
            )
        );
        self.render_buffer.extend(
            self.ecs_world.dead_tree_archetype.iter().map(|it|
                (Object::Stump, it.0)
            )
        );
        self.render_buffer.extend(
            self.ecs_world.store_house_archetype.iter().map(|it|
                (Object::BigHouse, it.0)
            )
        );
        self.render_buffer.sort_by(|l, r| {
            match l.1.1.cmp(&r.1.1) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => l.1.0.cmp(&r.1.0),
                Ordering::Greater => Ordering::Greater
            }
        });

        for (object, (x, y)) in self.render_buffer.drain(..) {
            object.blit(x as i32, y as i32);
        }
    }

    fn clear_visit_set(&mut self) {
        self.visited_fields.fill(0);
    }

    fn is_passable(&self, coord: (u8, u8)) -> bool {
        !self.obstacles.test(coord.1 as usize * 40 + coord.0 as usize)
    }

    fn get_next_path_dir(&self, coord: (u8, u8)) -> Direction {
        self.path_buffer[coord.1 as usize * 40 + coord.0 as usize]
    }

    fn set_visited(&mut self, coord: (u8, u8), dir: Direction) {
        let idx = coord.1 as usize * 40 + coord.0 as usize;
        self.visited_fields.set(idx);
        self.path_buffer[idx] = dir;
    }

    fn is_visited(&self, coord: (u8, u8)) -> bool {
        let idx = coord.1 as usize * 40 + coord.0 as usize;
        self.visited_fields.test(idx)
    }

    // breadth first search
    fn find_path(&mut self, start: (u8, u8), end: (u8, u8), out_vec: &mut Vec<Direction>) {
        if !self.is_passable(start) || !self.is_passable(end) {
            return;
        }
        if start.0 == end.0 && start.1 == end.1 {
            return;
        }
        self.clear_visit_set();
        self.path_buffer.fill(Direction::None);
        self.path_queue.clear();
        self.path_queue.push_front((end, Direction::None));
        while let Some((next_pos, dir)) = self.path_queue.pop_back() {
            let (cx, cy) = next_pos;
            if self.is_visited(next_pos) { continue; }
            self.set_visited(next_pos, dir);
            if cx == start.0 && cy == start.1 { break; }
            for (cond, pos, dir) in [
                (cx > 0, (cx - 1, cy), Direction::Right),
                (cx < 39, (cx + 1, cy), Direction::Left),
                (cy > 0, (cx, cy - 1), Direction::Down),
                (cy < 39, (cx, cy + 1), Direction::Up)
            ] {
                if cond && self.is_passable(pos) && !self.is_visited(pos) {
                    self.path_queue.push_front((pos, dir));
                }
            }
        }
        let (mut cur_x, mut cur_y) = start;
        while cur_x != end.0 || cur_y != end.1 {
            match self.get_next_path_dir((cur_x, cur_y)) {
                Direction::None => { return; },
                Direction::Up => {
                    out_vec.push(Direction::Up);
                    cur_y -= 1;
                },
                Direction::Down => {
                    out_vec.push(Direction::Down);
                    cur_y += 1;
                },
                Direction::Left => {
                    out_vec.push(Direction::Left);
                    cur_x -= 1;
                },
                Direction::Right => {
                    out_vec.push(Direction::Right);
                    cur_x += 1;
                },
            }
        }
    }
}