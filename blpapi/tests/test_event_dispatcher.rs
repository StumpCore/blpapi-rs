use blpapi::event_dispatcher::*;

#[test]
fn test_event_dispatcher_default() {
    let disp = EventDispatcherBuilder::default();
    assert_eq!(disp.num_dispatcher_threads, 1);
}

#[test]
fn test_event_dispatcher_create() {
    let no = 4usize;
    let disp = EventDispatcherBuilder::new(no);
    assert_eq!(disp.num_dispatcher_threads, 4);
}

#[test]
fn test_event_dispatcher_start() {
    let disp = EventDispatcherBuilder::default().build().start();
    assert!(disp.is_ok());
}

#[test]
fn test_event_dispatcher_stop() {
    let async_ = true;
    let disp = EventDispatcherBuilder::default().build();
    let start = disp.start();
    assert!(start.is_ok());
    let stop = disp.stop(&async_);
    assert!(stop.is_ok());
}

#[test]
fn test_event_dispatcher_dispatch_events() {
    let disp = EventDispatcherBuilder::default().build();
    let start = disp.start();
    assert!(start.is_ok());
    let res = disp.dispatch_events();
    assert!(res.is_ok());
}
