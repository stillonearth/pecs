use super::*;

pub fn exit() -> Promise<(), ()> {
    Promise::register(
        |_, _| {
            std::process::exit(0);
        },
        |_, _| {},
    )
}
