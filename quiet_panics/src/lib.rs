use std::panic::{set_hook, PanicHookInfo};

pub fn set_panic_hook() {
    if cfg!(not(debug_assertions)) {
        set_hook(Box::new(panic_hook));
    }
}

#[cfg(feature = "panic_payload_as_str")]
fn get_panic_message<'a>(info: &'a PanicHookInfo) -> Option<&'a str> {
    info.payload_as_str()
}

#[cfg(not(feature = "panic_payload_as_str"))]
fn get_panic_message<'a>(info: &'a PanicHookInfo) -> Option<&'a str> {
    match (
        info.payload().downcast_ref::<&str>(),
        info.payload().downcast_ref::<String>(),
    ) {
        (Some(s), _) => Some(*s),
        (_, Some(s)) => Some(s),
        (None, None) => None,
    }
}

fn panic_hook(info: &PanicHookInfo) {
    let message = get_panic_message(info);
    message.map_or_else(|| log::error!("{info}"), |message| log::error!("{message}"));
}
