#pragma once

#include <LibCore/EventLoop.h>
#include <LibCore/Timer.h>
#include <rust/cxx.h>
#include <memory>

inline std::unique_ptr<Core::EventLoop> make_event_loop()
{
    return std::make_unique<Core::EventLoop>();
}

class TimerWrapper
{
public:
    Core::Timer& pin_mut() { return *m_impl; }
    Core::Timer const& pin() const { return *m_impl; }

    explicit TimerWrapper(NonnullRefPtr<Core::Timer> impl) : m_impl(std::move(impl)) {}
private:
    NonnullRefPtr<Core::Timer> m_impl;    
};

inline std::unique_ptr<TimerWrapper> create_repeating_timer(i32 ms, rust::Fn<void()> handler)
{
    return std::make_unique<TimerWrapper>(MUST(Core::Timer::create_repeating(ms, [func = move(handler)]{
        func();
    })));
}

inline std::unique_ptr<TimerWrapper> create_single_shot_timer(i32 ms, rust::Fn<void()> handler)
{
    return std::make_unique<TimerWrapper>(MUST(Core::Timer::create_single_shot(ms, [func = move(handler)]{
        func();
    })));
}
