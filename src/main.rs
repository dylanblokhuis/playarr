use egui_glow::glow::HasContext;
use egui_glow::EguiGlow;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use libmpv::{render::RenderContext, Mpv};
use std::ffi::{c_char, c_void, CStr};
use std::mem::transmute;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

mod ui;

unsafe extern "C" fn get_proc_addr(ctx: *mut c_void, name: *const c_char) -> *mut c_void {
    let rust_name = CStr::from_ptr(name).to_str().unwrap();
    #[allow(clippy::transmute_ptr_to_ref)]
    let window: &&ContextWrapper<PossiblyCurrent, Window> = transmute(ctx);

    window.get_proc_address(rust_name) as *mut _
}

#[derive(Debug)]
enum UserEvent {
    MpvEventAvailable,
    RedrawRequested,
}

fn main() {
    unsafe {
        let (gl, _, window, event_loop) = {
            let event_loop = glutin::event_loop::EventLoop::<UserEvent>::with_user_event();
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Playarr")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .with_hardware_acceleration(Some(true))
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();

            let gl = egui_glow::painter::Context::from_loader_function(|s| {
                window.get_proc_address(s) as *const _
            });
            (gl, "#version 410", window, event_loop)
        };

        let mut mpv = Mpv::new().expect("Error while creating MPV");

        let mut render_context = RenderContext::new(mpv.ctx.as_mut(), &window, get_proc_addr)
            .expect("Failed creating render context");

        mpv.event_context_mut().disable_deprecated_events().unwrap();
        let event_proxy = event_loop.create_proxy();
        render_context.set_update_callback(move || {
            event_proxy.send_event(UserEvent::RedrawRequested).unwrap();
        });
        let event_proxy = event_loop.create_proxy();
        mpv.event_context_mut().set_wakeup_callback(move || {
            event_proxy
                .send_event(UserEvent::MpvEventAvailable)
                .unwrap();
        });

        mpv.set_property("video-timing-offset", 0).unwrap();

        let gl = Rc::new(gl);
        let mut egui_glow = EguiGlow::new(window.window(), gl.clone());
        let mpv = Arc::new(RwLock::new(mpv));
        let mut app = ui::App::new(mpv.clone(), &egui_glow.egui_ctx);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => (),
                Event::MainEventsCleared => {
                    window.window().request_redraw();
                }
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => {
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            window.resize(physical_size);
                        }
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => {}
                    }
                    egui_glow.on_event(&event);
                    window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    egui_glow.run(window.window(), |egui_ctx| app.render(egui_ctx));

                    let size = window.window().inner_size();
                    render_context
                        .render(size.width as i32, size.height as i32)
                        .expect("Failed to draw on glutin window");
                    egui_glow.paint(window.window());
                    gl.disable(glow::FRAMEBUFFER_SRGB);
                    gl.disable(glow::BLEND);
                    window.swap_buffers().unwrap();
                }
                Event::UserEvent(UserEvent::RedrawRequested) => {
                    render_context.update();
                    window.window().request_redraw();
                }
                Event::UserEvent(UserEvent::MpvEventAvailable) => loop {
                    match mpv.write().unwrap().event_context_mut().wait_event(0.0) {
                        Some(Ok(mpv_event)) => {
                            println!("MPV event: {mpv_event:?}");

                            match mpv_event {
                                libmpv::events::Event::PlaybackRestart => {
                                    app.playback = true
                                }
                                libmpv::events::Event::EndFile(_) => {
                                    app.playback = false
                                },
                                libmpv::events::Event::PropertyChange { name, change, reply_userdata } => {
                                    println!("Property change: {name:?} {change:?} {reply_userdata:?}");
                                }
                                _ => (),
                            }
                        }
                        Some(Err(err)) => {
                            println!("MPV Error: {err}");
                            *control_flow = ControlFlow::Exit;
                            break;
                        }
                        None => {
                            *control_flow = ControlFlow::Wait;
                            break;
                        }
                    }
                },

                _ => (),
            }
        });
    }
}
