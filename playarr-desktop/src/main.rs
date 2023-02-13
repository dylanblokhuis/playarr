#![windows_subsystem = "windows"]
use egui_glow::egui_winit::winit::event::{Event, WindowEvent};
use egui_glow::egui_winit::winit::event_loop::{ControlFlow, EventLoop};

use libmpv::Format;
use libmpv::{render::RenderContext, Mpv};
use std::ffi::{c_char, c_void, CStr};
use std::mem::transmute;
use std::sync::{Arc, Mutex};

mod pages;
mod server;
mod ui;
mod utils;
mod widgets;

use egui_glow::egui_winit::winit;

/// The majority of `GlutinWindowContext` is taken from `eframe`
struct GlutinWindowContext {
    window: winit::window::Window,
    gl_context: glutin::context::PossiblyCurrentContext,
    gl_display: glutin::display::Display,
    gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
}

impl GlutinWindowContext {
    // refactor this function to use `glutin-winit` crate eventually.
    // preferably add android support at the same time.
    #[allow(unsafe_code)]
    unsafe fn new(winit_window: winit::window::Window) -> Self {
        use glutin::prelude::*;
        use raw_window_handle::*;

        let raw_display_handle = winit_window.raw_display_handle();
        let raw_window_handle = winit_window.raw_window_handle();

        // EGL is crossplatform and the official khronos way
        // but sometimes platforms/drivers may not have it, so we use back up options where possible.

        // try egl and fallback to windows wgl. Windows is the only platform that *requires* window handle to create display.
        #[cfg(target_os = "windows")]
        let preference = glutin::display::DisplayApiPreference::EglThenWgl(Some(raw_window_handle));
        // try egl and fallback to x11 glx
        #[cfg(target_os = "linux")]
        let preference = glutin::display::DisplayApiPreference::EglThenGlx(Box::new(
            winit::platform::unix::register_xlib_error_hook,
        ));
        #[cfg(target_os = "macos")]
        let preference = glutin::display::DisplayApiPreference::Cgl;
        #[cfg(target_os = "android")]
        let preference = glutin::display::DisplayApiPreference::Egl;

        let gl_display = glutin::display::Display::new(raw_display_handle, preference).unwrap();

        let config_template = glutin::config::ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_depth_size(0)
            .with_stencil_size(0)
            .with_transparency(false)
            .compatible_with_native_window(raw_window_handle)
            .build();

        let config = gl_display
            .find_configs(config_template)
            .unwrap()
            .next()
            .unwrap();

        let context_attributes =
            glutin::context::ContextAttributesBuilder::new().build(Some(raw_window_handle));

        // for surface creation.
        let (width, height): (u32, u32) = winit_window.inner_size().into();
        let surface_attributes =
            glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
                .build(
                    raw_window_handle,
                    std::num::NonZeroU32::new(width).unwrap(),
                    std::num::NonZeroU32::new(height).unwrap(),
                );
        // start creating the gl objects
        let gl_context = gl_display
            .create_context(&config, &context_attributes)
            .unwrap();

        let gl_surface = gl_display
            .create_window_surface(&config, &surface_attributes)
            .unwrap();

        let gl_context = gl_context.make_current(&gl_surface).unwrap();

        // with_vsync
        gl_surface
            .set_swap_interval(
                &gl_context,
                glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap()),
            )
            .unwrap();

        GlutinWindowContext {
            window: winit_window,
            gl_context,
            gl_display,
            gl_surface,
        }
    }

    fn window(&self) -> &winit::window::Window {
        &self.window
    }

    fn resize(&self, physical_size: winit::dpi::PhysicalSize<u32>) {
        use glutin::surface::GlSurface;
        self.gl_surface.resize(
            &self.gl_context,
            physical_size.width.try_into().unwrap(),
            physical_size.height.try_into().unwrap(),
        );
    }

    fn swap_buffers(&self) -> glutin::error::Result<()> {
        use glutin::surface::GlSurface;
        self.gl_surface.swap_buffers(&self.gl_context)
    }

    fn get_proc_address(&self, addr: &std::ffi::CStr) -> *const std::ffi::c_void {
        use glutin::display::GlDisplay;
        self.gl_display.get_proc_address(addr)
    }
}

fn create_display(event_loop: &EventLoop<UserEvent>) -> (GlutinWindowContext, glow::Context) {
    let winit_window = winit::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(winit::dpi::LogicalSize {
            width: 1280.0,
            height: 720.0,
        })
        .with_title("Playarr")
        .with_visible(false) // Keep hidden until we've painted something. See https://github.com/emilk/egui/pull/2279
        .build(event_loop)
        .unwrap();

    // a lot of the code below has been lifted from glutin example in their repo.
    let glutin_window_context = unsafe { GlutinWindowContext::new(winit_window) };
    let gl = unsafe {
        glow::Context::from_loader_function(|s| {
            let s = std::ffi::CString::new(s)
                .expect("failed to construct C string from string for gl proc address");

            glutin_window_context.get_proc_address(&s)
        })
    };

    (glutin_window_context, gl)
}

unsafe extern "C" fn get_proc_addr(ctx: *mut c_void, name: *const c_char) -> *mut c_void {
    #[allow(clippy::transmute_ptr_to_ref)]
    let window: &&GlutinWindowContext = transmute(ctx);

    window.get_proc_address(CStr::from_ptr(name)) as *mut _
}

#[derive(Debug)]
enum UserEvent {
    RedrawRequested,
    MpvEventAvailable,
}

fn main() {
    let clear_color = [0.1, 0.1, 0.1];

    let event_loop = winit::event_loop::EventLoopBuilder::<UserEvent>::with_user_event().build();
    let (gl_window, gl) = create_display(&event_loop);
    let gl = std::sync::Arc::new(gl);
    let mut egui_glow = egui_glow::EguiGlow::new(&event_loop, gl.clone(), None);

    egui_glow
        .egui_winit
        .set_pixels_per_point(gl_window.window().scale_factor() as f32);

    let mut mpv = Mpv::new().expect("Error while creating MPV");
    mpv.set_property("video-timing-offset", 0).unwrap();

    let mut render_context =
        RenderContext::new(unsafe { mpv.ctx.as_mut() }, &gl_window, get_proc_addr)
            .expect("Failed creating render context");

    let event_proxy = event_loop.create_proxy();
    render_context.set_update_callback(move || {
        event_proxy.send_event(UserEvent::RedrawRequested).unwrap();
    });

    let mut render_context = Some(render_context);

    let mut app = ui::App::new(&egui_glow.egui_ctx);
    let mut ev_ctx = mpv.create_event_context();
    ev_ctx
        .observe_property("time-pos", Format::Double, 0)
        .unwrap();
    ev_ctx.observe_property("pause", Format::Flag, 0).unwrap();
    ev_ctx
        .observe_property("demuxer-cache-state", Format::Node, 0)
        .unwrap();
    ev_ctx
        .observe_property("duration", Format::Double, 0)
        .unwrap();
    ev_ctx.observe_property("volume", Format::Int64, 0).unwrap();

    let event_proxy = event_loop.create_proxy();
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
        event_proxy
            .send_event(UserEvent::MpvEventAvailable)
            .unwrap();
    });

    let repaint_proxy = Arc::new(Mutex::new(event_loop.create_proxy()));
    egui_glow.egui_ctx.set_request_repaint_callback(move || {
        repaint_proxy
            .lock()
            .unwrap()
            .send_event(UserEvent::RedrawRequested)
            .ok();
    });

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            let repaint_after =
                egui_glow.run(gl_window.window(), |egui_ctx| app.render(egui_ctx, &mpv));
            if repaint_after.is_zero() {
                gl_window.window().request_redraw();
                ControlFlow::Poll
            } else if let Some(repaint_after_instant) =
                std::time::Instant::now().checked_add(repaint_after)
            {
                ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                ControlFlow::Wait
            };

            {
                unsafe {
                    use glow::HasContext as _;
                    gl.clear_color(clear_color[0], clear_color[1], clear_color[2], 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                let size = gl_window.window().inner_size();
                // draw things behind egui here
                render_context
                    .as_mut()
                    .unwrap()
                    .render(size.width as i32, size.height as i32)
                    .expect("Failed to draw on glutin window");
                egui_glow.paint(gl_window.window());

                // draw things on top of egui here

                gl_window.swap_buffers().unwrap();
                gl_window.window().set_visible(true);
            }
        };

        match event {
            Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            Event::RedrawRequested(_) if !cfg!(windows) => redraw(),
            Event::WindowEvent { event, .. } => {
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = ControlFlow::Exit;
                }

                if let WindowEvent::Resized(physical_size) = &event {
                    gl_window.resize(*physical_size);
                } else if let WindowEvent::ScaleFactorChanged { new_inner_size, .. } = &event {
                    gl_window.resize(**new_inner_size);
                }

                app.handle_player_keyboard_events(&event, &mpv);
                app.handle_player_mouse_events(&event, &mpv);

                let event_response = egui_glow.on_event(&event);
                if event_response.repaint {
                    gl_window.window().request_redraw();
                }
            }
            Event::UserEvent(UserEvent::RedrawRequested) => {
                render_context.as_ref().unwrap().update();
                gl_window.window().request_redraw();
            }
            Event::UserEvent(UserEvent::MpvEventAvailable) => match ev_ctx.wait_event(0.0) {
                Some(Ok(mpv_event)) => {
                    app.handle_mpv_events(&mpv_event);
                    gl_window.window.request_redraw();
                }
                Some(Err(err)) => {
                    println!("MPV Error: {err}");
                    *control_flow = ControlFlow::Exit;
                }
                None => {
                    *control_flow = ControlFlow::Wait;
                }
            },
            Event::LoopDestroyed => {
                render_context.take();
                egui_glow.destroy();
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
