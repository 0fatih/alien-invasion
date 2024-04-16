use std::process::exit;

use clap::Parser;
use tracing::{debug, info, trace};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod alien;
mod error;
mod world;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    map_file: String,
    alien_count: usize,
    iterations: usize,
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "alien_task=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Initializing world");
    let mut world = world::World::new();

    let file = std::fs::read_to_string(args.map_file).expect("Failed to read map file");
    world.load_map(file.clone());
    world.spawn_aliens(args.alien_count);

    // start invasion
    for i in 1..=args.iterations {
        info!("Day {}", i);
        world.collide();
        world.day_and_night();

        if world.aliens.iter().all(|a| a.is_dead || a.is_trapped) {
            info!("All aliens are dead or trapped. Ending simulation.");
            exit(0);
        }

        if world.cities.len() == 0 {
            info!("All cities are destroyed. Poor humans.");
            exit(0);
        }
    }

    info!("Humanity survived the alien invasion.")
}
