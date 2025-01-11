use input::{Libinput, LibinputInterface, Device, Event};
use input::event::{EventTrait, PointerEvent};
use input::event::pointer::{PointerButtonEvent, PointerScrollWheelEvent};

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<i32, i32> {
        unsafe {
            let fd = libc::open(path.as_ptr() as *const libc::c_char, flags);
            if fd < 0 {
                Err(*libc::__errno_location())
            } else {
                Ok(fd)
            }
        }
    }

    fn close_restricted(&mut self, fd: i32) {
        unsafe {
            libc::close(fd);
        }
    }
}

fn main() {
    let mut interface = Interface;
    let context = Libinput::new_with_udev(&mut interface);
    context.udev_assign_seat("seat0").unwrap();

    loop {
        context.dispatch().unwrap();
        for event in context.by_ref() {
            if let Event::Device(device_event) = event {
                let device = device_event.device();
                if device.has_capability(input::DeviceCapability::Pointer) {
                    handle_pointer_event(event)
                }
            }
        }
    }
}

fn handle_pointer_event(event: Event) {
    match event {
        Event::Pointer(pointer_event) => {
            match pointer_event {
                PointerEvent::Button(button_event) => {
                    let time = button_event.time_usec();
                    println!("Button event: {:?} at {}", button_event, time);
                }
                PointerEvent::ScrollWheel(scroll_wheel_event) => {
                    let time = scroll_wheel_event.time_usec();
                    println!("Scroll wheel event: {:?} at {}", scroll_wheel_event, time);
                }
                _ => {}
            }
        }
        _ => {}
    }
}