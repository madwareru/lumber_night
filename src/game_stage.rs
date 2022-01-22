use std::cmp::Ordering;
use std::collections::VecDeque;
use crate::objects::{Object};
use crate::random::RndGen;
use crate::tilemap::Tilemap;
use super::wasm4::*;
use bitsetium::{BitEmpty, BitSet, BitTest};

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

#[derive(Copy, Clone)]
enum WorkerState {
    Idle,
    WalkingToTree { steps_to_next_dir: u8 },
    ChoppingTree { ticks_to_next_chop: u8 },
    CarryCargoToStorage { steps_to_next_dir: u8 },
    GettingRidOfCargo { ticks_to_next_state: u8 }
}

#[derive(Copy, Clone)]
struct TargetTree {
    tree_id: usize, tree_x: i16, tree_y: i16
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

const BIG_HOUSE_DOOR_COORDS: (u8, u8) = (15, 6);
const GROW_TICKS: usize = 32;
const WATER_ANIMATION_TICKS: usize = 10;
const CHOP_TICKS: u8 = 48;
const STEP_SUB_TICKS: u8 = 4;
const OAK_AGE_TILL_MATURE: u16 = 100;
const PINE_AGE_TILL_MATURE: u16 = 20;
const PINE_HP_MAX: u8 = 15;
const OAK_HP_MAX: u8 = 21;

struct World {
    pub workers: Vec<((i16, i16), Bag, VecDeque<Direction>, Option<TargetTree>, WorkerState)>,
    pub ripples: Vec<((i16, i16), RippleOffset)>,
    pub store: Vec<((i16, i16), StoreHouse)>,
    pub mature: Vec<(Object, (i16, i16), HealthPoints )>,
    pub young: Vec<(Object, (i16, i16), AgePoints )>,
    pub dead: Vec<((i16, i16), )>,
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
    current_frame: usize,
    world: World,
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
            path_queue: VecDeque::new(),
            world: World {
                workers: Vec::new(),
                ripples: Vec::new(),
                store: Vec::new(),
                mature: Vec::new(),
                young: Vec::new(),
                dead: Vec::new()
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
                    self.world.ripples.push(((x, y), RippleOffset(off_x, off_y)));
                    last_set = true;
                }
            }
        }


        for &(object, (x, y)) in OBJECTS.iter() {
            match object {
                Object::BigHouse => {
                    let yy = (y / 4) as usize;
                    let xx = (x / 4) as usize;
                    for j in yy - 1 .. yy + 4 {
                        for i in xx - 1 .. xx + 4 {
                            if j == yy - 1 && i != xx && i != xx + 1 { continue; }
                            if j == yy + 3 && i != xx + 1 { continue; }
                            let passability_tile_idx = j * 40 + i;
                            self.obstacles.set(passability_tile_idx);
                        }
                    }
                    self.world.store.push(
                        ((x, y), StoreHouse { count_items: 0 })
                    );
                }
                Object::Worker => {
                    self.world.workers.push(
                        (
                            (x, y),
                            Bag {
                                count_items: 0,
                                max_items: 3
                            },
                            VecDeque::new(),
                            Option::None::<TargetTree>,
                            WorkerState::Idle
                        )
                    );
                }
                _ => {
                    let passability_tile_idx = (y / 4) as usize * 40 + (x / 4) as usize;
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
                            self.world.mature.push(
                                (
                                    object,
                                    (x, y),
                                    HealthPoints { amount: hp }
                                )
                            );
                        } else {
                            self.world.young.push(
                                (
                                    object,
                                    (x, y),
                                    AgePoints { amount: 0 }
                                )
                            )
                        }
                    } else {
                        self.world.dead.push(
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
        self.murder_trees();
        if self.current_frame % GROW_TICKS == 0 {
            self.grow_trees();
        }
        self.update_workers();
        self.current_frame += 1;
    }

    fn murder_trees(&mut self) {
        for mature_i in (0..self.world.mature.len()).rev() {
            let (_, position, health) = self.world.mature[mature_i];
            if health.amount == 0 {
                self.world.mature.swap_remove(mature_i);
                self.world.dead.push((position,));
            }
        }
    }

    fn grow_trees(&mut self) {
        let mut rng = std::mem::take(&mut self.rnd_gen);

        // make some living for long dead trees:
        {
            for stump_i in (0..self.world.dead.len()).rev() {
                if rng.gen_range(0..=1000) < 2 {
                    let ((x, y),) = self.world.dead.swap_remove(stump_i);
                    self.world.young.push(
                        (
                            if rng.gen_range(0..=9) >= 7 { Object::LittleOak } else { Object::LittlePine },
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
                for young_i in (0..self.world.young.len()).rev() {
                    let (object, position, age_pts) = self.world.young[young_i];
                    match (object, age_pts) {
                        (Object::LittleOak, age_points) if age_points.amount >= OAK_AGE_TILL_MATURE => {
                            self.world.young.swap_remove(young_i);
                            self.world.mature.push(
                                (
                                    Object::Oak,
                                    position,
                                    HealthPoints { amount: OAK_HP_MAX }
                                )
                            );
                        },
                        (Object::LittlePine, age_points) if age_points.amount >= PINE_AGE_TILL_MATURE => {
                            self.world.young.swap_remove(young_i);
                            self.world.mature.push(
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
                for (obj, _, age_pts ) in self.world.young.iter_mut() {
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
        for ripple in self.world.ripples.iter_mut() {
            if rng.gen_range(0..=99) > 40 {
                continue;
            }
            let RippleOffset(_, off_y) = ripple.1;
            let off_x = rng.gen_range_i(-1..=1) as i8;
            ripple.1 = RippleOffset(off_x, off_y);
        }
        self.rnd_gen = rng;
    }

    fn update_workers(&mut self) {
        let mut rng = std::mem::take(&mut self.rnd_gen);
        let mut trees = std::mem::take(&mut self.world.mature);
        let mut workers = std::mem::take(&mut self.world.workers);

        let mut tree_indices: [Option<usize>; 3] = [None; 3];

        // actualize tree indices for workers
        {
            for i in 0..workers.len() {
                if let Some(tree_data) = workers[i].3 {
                    if tree_data.tree_id >= trees.len()
                        || tree_data.tree_x != trees[tree_data.tree_id].1.0
                        || tree_data.tree_y != trees[tree_data.tree_id].1.1 {
                        // something is wrong! We will try to find index!
                        workers[i].3 = trees
                            .iter()
                            .enumerate()
                            .find(|it| it.1.1.0 == tree_data.tree_x && it.1.1.1 == tree_data.tree_y)
                            .map(|it| TargetTree { tree_id: it.0, ..tree_data });
                    }
                }
                tree_indices[i] = workers[i].3.map(|it| it.tree_id);
            }
        }

        for (id, worker) in workers.iter_mut().enumerate() {
            let state = worker.4;
            match state {
                WorkerState::Idle => {
                    let worker_pos = ((worker.0.0 / 4) as u8, (worker.0.1 / 4) as u8);
                    let mut path_buf = std::mem::take(&mut worker.2);

                    // there is a possibility when worker already has a tree,
                    // in this case we just need to find a path back to it
                    if let Some(tree_id) = tree_indices[id] {
                        let (tx, ty) = (trees[tree_id].1.0 / 4, trees[tree_id].1.1 / 4);
                        'outer: for x in tx-1 ..= tx+1 {
                            if x < 0 || x > 39 { continue; }
                            for y in ty-1 ..= ty+1 {
                                if y < 0 || y > 39 { continue; }
                                self.find_path(worker_pos, (x as u8, y as u8), &mut path_buf);
                                if !path_buf.is_empty() {
                                    worker.4 = WorkerState::WalkingToTree {
                                        steps_to_next_dir: STEP_SUB_TICKS
                                    };
                                    break 'outer;
                                }
                            }
                        }
                    } else {
                        // just pick a random tree and see if we have a path towards it
                        'search: loop {
                            let random_id = rng.gen_range(0..=(trees.len() - 1) as u16) as usize;
                            if trees[random_id].2.amount == 0 {
                                continue;
                            }
                            if tree_indices.iter().enumerate().any(|(i, v)| {
                                i != id && match v {
                                    None => false,
                                    Some(idx) => *idx == random_id
                                }
                            }) {
                                continue;
                            }
                            let (tx, ty) = (trees[random_id].1.0 / 4, trees[random_id].1.1 / 4);

                            for x in tx-1 ..= tx+1 {
                                if x < 0 || x > 39 { continue; }
                                for y in ty-1 ..= ty+1 {
                                    if y < 0 || y > 39 { continue; }
                                    self.find_path(worker_pos, (x as u8, y as u8), &mut path_buf);
                                    if !path_buf.is_empty() {
                                        worker.3 = Some(
                                            TargetTree {
                                                tree_id: random_id,
                                                tree_x: trees[random_id].1.0,
                                                tree_y: trees[random_id].1.1
                                            }
                                        );
                                        worker.4 = WorkerState::WalkingToTree {
                                            steps_to_next_dir: STEP_SUB_TICKS
                                        };
                                        break 'search;
                                    }
                                }
                            }
                        }
                    }
                    worker.2 = path_buf;
                },
                WorkerState::WalkingToTree { steps_to_next_dir } => {
                    if steps_to_next_dir == 0 {
                        worker.2.pop_front();
                        if worker.2.is_empty() {
                            // we reached a destination
                            worker.4 = WorkerState::ChoppingTree {
                                ticks_to_next_chop: CHOP_TICKS
                            };
                            continue;
                        }
                        worker.4 = WorkerState::WalkingToTree {
                            steps_to_next_dir: STEP_SUB_TICKS
                        }
                    } else {
                        match worker.2.front() {
                            Some(Direction::Left) => {
                                worker.0.0 -= 1;
                            },
                            Some(Direction::Right) => {
                                worker.0.0 += 1;
                            },
                            Some(Direction::Up) => {
                                worker.0.1 -= 1;
                            },
                            Some(Direction::Down) => {
                                worker.0.1 += 1;
                            },
                            _ => ()
                        }

                        worker.4 = WorkerState::WalkingToTree {
                            steps_to_next_dir: steps_to_next_dir-1
                        }
                    }
                },
                WorkerState::ChoppingTree { ticks_to_next_chop } => {
                    if ticks_to_next_chop == 0 {
                        match tree_indices[id] {
                            None => {
                                // seems our tree is dead, nothing to do, move to store
                                let mut path_buf = std::mem::take(&mut worker.2);
                                self.find_path(
                                    ((worker.0.0 / 4) as u8, (worker.0.1 / 4) as u8),
                                    BIG_HOUSE_DOOR_COORDS,
                                    &mut path_buf
                                );
                                worker.4 = WorkerState::CarryCargoToStorage {
                                    steps_to_next_dir: STEP_SUB_TICKS
                                };
                                worker.2 = path_buf;
                            }
                            Some(tree_id) => {
                                let hp = trees[tree_id].2;
                                if hp.amount == 0 {
                                    let mut path_buf = std::mem::take(&mut worker.2);
                                    self.find_path(
                                        ((worker.0.0 / 4) as u8, (worker.0.1 / 4) as u8),
                                        BIG_HOUSE_DOOR_COORDS,
                                        &mut path_buf
                                    );
                                    worker.4 = WorkerState::CarryCargoToStorage {
                                        steps_to_next_dir: STEP_SUB_TICKS
                                    };
                                    worker.2 = path_buf;
                                } else {
                                    trees[tree_id].2 = HealthPoints { amount: hp.amount - 1 };
                                    worker.1.count_items += 1;
                                    if worker.1.count_items == worker.1.max_items {
                                        // lumberjack filled it's bag
                                        let mut path_buf = std::mem::take(&mut worker.2);
                                        self.find_path(
                                            ((worker.0.0 / 4) as u8, (worker.0.1 / 4) as u8),
                                            BIG_HOUSE_DOOR_COORDS,
                                            &mut path_buf
                                        );
                                        worker.4 = WorkerState::CarryCargoToStorage {
                                            steps_to_next_dir: STEP_SUB_TICKS
                                        };
                                        worker.2 = path_buf;
                                    }
                                }
                            }
                        }
                    } else {
                        worker.4 = WorkerState::ChoppingTree {
                            ticks_to_next_chop: ticks_to_next_chop - 1
                        };
                    }
                },
                WorkerState::CarryCargoToStorage { steps_to_next_dir } => {
                    if steps_to_next_dir == 0 {
                        worker.2.pop_front();
                        if worker.2.is_empty() {
                            // we reached a destination
                            worker.4 = WorkerState::GettingRidOfCargo {
                                ticks_to_next_state: CHOP_TICKS / 2
                            };
                            continue;
                        }
                        worker.4 = WorkerState::CarryCargoToStorage {
                            steps_to_next_dir: STEP_SUB_TICKS
                        }
                    } else {
                        match worker.2.front() {
                            Some(Direction::Left) => {
                                worker.0.0 -= 1;
                            },
                            Some(Direction::Right) => {
                                worker.0.0 += 1;
                            },
                            Some(Direction::Up) => {
                                worker.0.1 -= 1;
                            },
                            Some(Direction::Down) => {
                                worker.0.1 += 1;
                            },
                            _ => ()
                        }

                        worker.4 = WorkerState::CarryCargoToStorage {
                            steps_to_next_dir: steps_to_next_dir-1
                        }
                    }
                },
                WorkerState::GettingRidOfCargo { ticks_to_next_state } => {
                    if ticks_to_next_state > 0 {
                        worker.4 = WorkerState::GettingRidOfCargo {
                            ticks_to_next_state: ticks_to_next_state - 1
                        };
                    } else {
                        self.world.store[0].1.count_items += worker.1.count_items as u64;
                        worker.1.count_items = 0;
                        worker.4 = WorkerState::Idle;
                    }
                }
            }
        }
        self.world.workers = workers;
        self.world.mature = trees;
        self.rnd_gen = rng;
    }

    pub fn render(&mut self) {
        self.render_ripples();
        self.tilemap.draw();
        self.render_objects();
    }

    fn render_ripples(&self) {
        for ((x, y), RippleOffset(off_x, off_y)) in self.world.ripples.iter() {
            let x = *x as i32 + *off_x as i32;
            let y = *y as i32 + *off_y as i32;
            unsafe {*super::wasm4::DRAW_COLORS = 2;}
            super::wasm4::hline(x, y, 4);
        }
    }

    fn render_objects(&mut self) {
        self.render_buffer.extend(
            self.world.workers.iter().map(|it|
                (Object::Worker, it.0)
            )
        );
        self.render_buffer.extend(
            self.world.mature.iter().map(|it|
                (it.0, it.1)
            )
        );
        self.render_buffer.extend(
            self.world.young.iter().map(|it|
                (it.0, it.1)
            )
        );
        self.render_buffer.extend(
            self.world.dead.iter().map(|it|
                (Object::Stump, it.0)
            )
        );
        self.render_buffer.extend(
            self.world.store.iter().map(|it|
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
    fn find_path(&mut self, start: (u8, u8), end: (u8, u8), out_vec: &mut VecDeque<Direction>) {
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
                    out_vec.push_back(Direction::Up);
                    cur_y -= 1;
                },
                Direction::Down => {
                    out_vec.push_back(Direction::Down);
                    cur_y += 1;
                },
                Direction::Left => {
                    out_vec.push_back(Direction::Left);
                    cur_x -= 1;
                },
                Direction::Right => {
                    out_vec.push_back(Direction::Right);
                    cur_x += 1;
                },
            }
        }
    }
}