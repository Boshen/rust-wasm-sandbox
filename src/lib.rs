mod dom;
mod geometry;
mod gl;

mod game_of_life;
mod mendelbrot;
mod threed;
mod tracer;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
