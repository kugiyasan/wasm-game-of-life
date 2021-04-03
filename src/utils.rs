extern crate web_sys;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// A macro to provide `println!(..)`-style syntax for `console.log` logging
///
/// BE SURE to compile your code with `debug = true`
/// or else this macro won't do anything
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// When not in debug mode, `log` won't do anything
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! log {
    ( $( $t:tt )* ) => {}
}
