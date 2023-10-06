use crate::toxic::payload;

mod core;
mod toxic;

fn main() {
    let mut pe = core::PolymorphicEngine::new();

    pe.run(payload)
}
