use serenity_rust::ffi;

pub fn main() {
    let mut event_loop = ffi::make_event_loop();

    let mut timer = ffi::create_repeating_timer(1000, || eprintln!("Hello from rust!"));
    timer.pin_mut().pin_mut().start();

    event_loop.pin_mut().exec();
}
