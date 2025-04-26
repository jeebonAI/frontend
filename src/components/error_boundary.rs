use dioxus::prelude::*;

#[component]
pub fn ErrorBoundary(children: Element) -> Element {
    // Set up a global panic hook to catch errors
    use_hook(|| {
        std::panic::set_hook(Box::new(|panic_info| {
            let message = format!("{:?}", panic_info);
            log::error!("Panic occurred: {}", message);
        }));
    });

    // Just render the children for now
    // In a real app, we would implement proper error boundaries
    children
}
