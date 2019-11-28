use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::{fs::File, path::Path};

fn main() {
  let mut file = File::create(Path::new("bindings.rs")).unwrap();

  Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::Yes, [])
    .write_bindings(
      GlobalGenerator::default()
        .with_use_atomics(true)
        .with_trace(true)
        .with_debug_assert_error_check(true)
        .with_inner_attributes(true),
      &mut file,
    )
    .expect("Couldn't write to file!");
}
