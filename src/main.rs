use bgfx::*;
use bgfx_rs::bgfx;
use winit::{
    dpi::LogicalSize,
    keyboard::{PhysicalKey, KeyCode},
    event::{ElementState, Event, KeyEvent, WindowEvent, DeviceEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::PhysicalSize,
    window::Window,
};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn get_platform_data(window: &Window) -> PlatformData {
    let mut pd = PlatformData::new();

    match window.raw_window_handle().unwrap() {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(data) => {
            pd.nwh = data.window as *mut _;
        }
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(data) => {
            pd.ndt = data.surface; // same as window, on wayland there ins't a concept of windows
        }

        #[cfg(target_os = "macos")]
        RawWindowHandle::AppKit(data) => {
            pd.nwh = data.ns_window;
        }
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(data) => {
            pd.nwh = data.hwnd.get() as *const _;
        }
        _ => panic!("Unsupported Window Manager"),
    }

    return pd;
}

#[cfg(target_os = "linux")]
fn get_render_type() -> RendererType {
    RendererType::OpenGL
}

#[cfg(not(target_os = "linux"))]
fn get_render_type() -> RendererType {
    RendererType::Count
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Winit BGFX Example (Esc to exit)")
        .with_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let mut init = Init::new();

    init.type_r = get_render_type();
    init.resolution.width = WIDTH as u32;
    init.resolution.height = HEIGHT as u32;
    init.resolution.reset = ResetFlags::VSYNC.bits();
    init.platform_data = get_platform_data(&window);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    bgfx::set_debug(DebugFlags::TEXT.bits());
    bgfx::set_view_clear(
        0,
        ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
        SetViewClearArgs {
            rgba: 0x103030ff,
            ..Default::default()
        },
    );

    let mut old_size = PhysicalSize::new(0, 0);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(key),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => match key {
                KeyCode::Escape => {
                    elwt.exit();
                }
                _ => (),
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => elwt.exit(),
            _ => (),
        }

        let size = window.inner_size();

        if old_size != size {
            bgfx::reset(size.width as _, size.height as _, ResetArgs::default());
            old_size = size;
        }

        bgfx::set_view_rect(0, 0, 0, size.width as _, size.height as _);
        bgfx::touch(0);

        bgfx::dbg_text_clear(DbgTextClearArgs::default());

        bgfx::dbg_text(0, 1, 0x0f, "Color can be changed with ANSI \x1b[9;me\x1b[10;ms\x1b[11;mc\x1b[12;ma\x1b[13;mp\x1b[14;me\x1b[0m code too.");
        bgfx::dbg_text(80, 1, 0x0f, "\x1b[;0m    \x1b[;1m    \x1b[; 2m    \x1b[; 3m    \x1b[; 4m    \x1b[; 5m    \x1b[; 6m    \x1b[; 7m    \x1b[0m");
        bgfx::dbg_text(80, 2, 0x0f, "\x1b[;8m    \x1b[;9m    \x1b[;10m    \x1b[;11m    \x1b[;12m    \x1b[;13m    \x1b[;14m    \x1b[;15m    \x1b[0m");
        bgfx::dbg_text(
            0,
            4,
            0x3f,
            "Description: Initialization and debug text with bgfx-rs Rust API.",
        );

        bgfx::frame(false);
    });
}
