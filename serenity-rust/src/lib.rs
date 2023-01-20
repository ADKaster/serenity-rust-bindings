#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("LibCore/EventLoop.h");
        include!("Shim.h");

        #[namespace="Core"]
        type EventLoop;

        pub fn make_event_loop() -> UniquePtr<EventLoop>;
        pub fn exec(self: Pin<&mut EventLoop>) -> i32;

        #[namespace="Core"]
        type Timer;

        pub fn start(self: Pin<&mut Timer>);

        type TimerWrapper;
        pub fn create_single_shot_timer(ms : i32,  callback: fn()) -> UniquePtr<TimerWrapper>;
        pub fn create_repeating_timer(ms : i32,  callback: fn()) -> UniquePtr<TimerWrapper>;

        fn pin(self: &TimerWrapper) -> &Timer;
        fn pin_mut(self: Pin<&mut TimerWrapper>) -> Pin<&mut Timer>;
    }

}
