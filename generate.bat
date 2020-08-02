cargo install phosphorus
mkdir target
phosphorus ../gl.xml gles2 3 0 core GL_KHR_debug >src/lib.rs
cargo fmt
