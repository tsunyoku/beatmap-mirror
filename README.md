# beatmap-mirror

A fast, efficient osu! beatmap mirror written in asynchronous Rust. Supports cheesegull, aswell as osu!api v2 formats.

# Setup

You need elasticsearch. You can use the docker-compose file provided to setup a simple elasticsearch instance.

Then, build the project using `cargo build --release`. Configure your environment variables as per `src/config.rs`, and run.