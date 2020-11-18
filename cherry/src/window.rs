use crate::{
    event::Event,
    input::{
        button::Button,
        key::Key,
    },
};

use sdl2::{
    event::Event as SdlEvent,
    keyboard::Keycode as SdlKeycode,
    video::{
        GLContext,
        GLProfile,
        Window as SdlWindow,
    },
    EventPump as SdlEventPump,
    Sdl,
    VideoSubsystem as SdlVideo,
};

pub struct Window {
    events: SdlEventPump,
    window: SdlWindow,
    _sdl: Sdl,
    _video: SdlVideo,
    _gl: GLContext,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl = sdl2::init().expect("Failed to initialise SDL2.");

        let video = sdl
            .video()
            .expect("Failed to initialise the video subsystem.");

        let events = sdl
            .event_pump()
            .expect("Failed to initialise the event subsystem.");

        let gl_attributes = video.gl_attr();
        gl_attributes.set_context_profile(GLProfile::Core);
        gl_attributes.set_context_version(4, 5);

        let window = video
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to create window.");

        let gl = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Self {
            events,
            window,
            _sdl: sdl,
            _video: video,
            _gl: gl,
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window
            .set_size(width, height)
            .expect("Failed to set window size.");

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        if let Some(event) = self.events.poll_event() {
            match event {
                SdlEvent::KeyDown { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        let key = convert_keycode(keycode);
                        Some(Event::KeyDown { key })
                    } else {
                        None
                    }
                }
                SdlEvent::KeyUp { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        let key = convert_keycode(keycode);
                        Some(Event::KeyUp { key })
                    } else {
                        None
                    }
                }
                SdlEvent::MouseButtonDown { .. } => Some(Event::MouseButtonDown {
                    button: Button::Left,
                }),
                SdlEvent::MouseButtonUp { .. } => Some(Event::MouseButtonUp {
                    button: Button::Left,
                }),
                SdlEvent::MouseMotion {
                    x, y, xrel, yrel, ..
                } => Some(Event::MouseMove {
                    x,
                    y,
                    dx: xrel,
                    dy: yrel,
                }),
                SdlEvent::MouseWheel { y, .. } => {
                    let delta = y;
                    Some(Event::MouseScroll { delta })
                }
                SdlEvent::Quit { .. } => Some(Event::Quit),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.gl_swap_window();
    }
}

fn convert_keycode(keycode: SdlKeycode) -> Key {
    match keycode {
        SdlKeycode::Up => Key::Up,
        SdlKeycode::Down => Key::Down,
        SdlKeycode::Return => Key::Enter,
        SdlKeycode::Backspace => Key::Backspace,
        _ => Key::Unknown,
    }
}
