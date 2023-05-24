use std::panic::PanicInfo;

pub fn set_panic_hook() {
    if cfg!(not(debug_assertions)) {
        std::panic::set_hook(Box::new(panic_hook));
    }
}

fn panic_hook(info: &PanicInfo) {
    #[cfg(feature = "nightly")]
    let message = panic_info.message().map(|m| format!("{}", m));

    #[cfg(not(feature = "nightly"))]
    let message = match (
        info.payload().downcast_ref::<&str>(),
        info.payload().downcast_ref::<String>(),
    ) {
        (Some(s), _) => Some((*s).to_owned()),
        (_, Some(s)) => Some(s.to_string()),
        (None, None) => None,
    };

    message.map_or_else(
        || log::error!("{}", info),
        |message| log::error!("{}", message),
    );
}
