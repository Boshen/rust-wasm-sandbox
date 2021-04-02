mod dom;
mod threed;
mod webgl;

mod game_of_life;
mod mendelbrot;
mod tracer;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
