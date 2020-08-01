cargo install phosphorus
mkdir target
phosphorus ../gl.xml gles2 3 0 core >target/lib.rs
rustfmt target/lib.rs
