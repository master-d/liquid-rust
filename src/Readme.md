# Liquid Rust
liquid-rust is version of an old commodore 64 game written using in rust.  
It uses google's liqid fun crate to simulate 2d physics for rigid objects like 
the dam and liquid physics for the water behind the dam.  It also uses SDL2 to 
render objects created in the box2d world using opengl
### Compiling
install the rustc compiler and rust's package manager cargo for your OS.
You will also need to install the c library for sdl2_gfx since it appears
that this has not been ported to rust yet.

example for gentoo Linux:
`emerge -av rust-bin cargo sdl2-gfx`
`cd /directory/of/git/clone`
`cargo build --release`

**binaries will be stored in ./target/release 
### Status
Incomplete: The application will build a dam using 20x20 pixel blocks and pour
water behind the dam.  <space> can be pressed to drop blocks onto the dam to 
destroy it.