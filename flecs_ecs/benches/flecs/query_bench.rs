#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(warnings)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flecs_ecs::prelude::*;
use seq_macro::seq;
use std::{ffi::CStr, sync::OnceLock};

#[derive(Debug, Component, Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component, Clone)]
pub struct Vel {
    pub x: f32,
    pub y: f32,
}

seq!(P in 0..=20 {
    // expands to structs named x0, x1, x2, ..., 20
    #[derive(Debug,   Component)]
    struct X~P
    {
        x: f32,
        y: f32,
    }
});

fn flip_coin() -> bool {
    rand::random::<bool>()
}

fn query_each_benchmark(c: &mut Criterion) {
    // Setup world and entities
    let world = World::new();

    for _ in 0..1000_000 {
        let mut e = world.entity();
        e.set(Pos { x: 10.0, y: 20.0 });
        e.set(Vel { x: 5.0, y: 5.0 });
        if flip_coin() {
            e.set(X2 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X3 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X4 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X5 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X6 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X7 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X8 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X9 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X10 { x: 0.0, y: 0.0 });
        }
        if flip_coin() {
            e.set(X11 { x: 0.0, y: 0.0 });
        }
    }

    let mut query = world.query::<(&mut Pos, &Vel)>().set_cached().build();

    c.bench_function("query_each", |b| {
        b.iter(|| {
            let mut counter = 0;
            query.each(|(pos, vel)| {
                counter += 1;
                pos.x += vel.x;
                pos.y += vel.y;
            });

            // This will make sure the benchmarked code isn't optimized away
            black_box(counter);
        });
    });
}
