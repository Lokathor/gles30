cargo install phosphorus
mkdir target
phosphorus ../gl.xml gles2 4 6 core GL_KHR_debug >src/lib.rs
cargo fmt
