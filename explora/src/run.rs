use crate::event::WindowEvent;
use crate::{event::Events, input::Input, App};
use apecs::Write;
use common::state::SysResult;
use log::info;
use render::{GpuGlobals, Renderer};
use vek::Vec2;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
};

pub fn run(event_loop: EventLoop<()>, mut app: App) {
    info!("Running explora");
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, elwt| match event {
            winit::event::Event::AboutToWait => {
                let events = app.state.resource_mut::<Events<WindowEvent>>();
                events.clear();
                app.window.platform().request_redraw();
            },
            winit::event::Event::WindowEvent { event, window_id }
                if window_id == app.window.platform().id() =>
            {
                match event {
                    winit::event::WindowEvent::RedrawRequested => {
                        let globals = *app.state.resource::<GpuGlobals>();
                        let renderer = app.state.resource_mut::<Renderer>();
                        renderer.write_globals(globals);
                        app.state.tick(app.clock.dt());
                        app.clock.tick();
                    },
                    winit::event::WindowEvent::CloseRequested => elwt.exit(),
                    winit::event::WindowEvent::Resized(size) => {
                        let renderer = app.state.resource_mut::<Renderer>();
                        renderer.resize(size.width, size.height);

                        let events = app.state.resource_mut::<Events<WindowEvent>>();
                        let new_size = Vec2::new(size.width, size.height);
                        events.send(WindowEvent::Resize(new_size));
                    },
                    winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                        let events = app.state.resource_mut::<Events<WindowEvent>>();
                        let size = app.window.platform().inner_size();
                        events.send(WindowEvent::Resize(Vec2::new(size.width, size.height)));
                    },
                    winit::event::WindowEvent::KeyboardInput { event, .. } => {
                        if let PhysicalKey::Code(code) = event.physical_key {
                            let input = app.state.resource_mut::<Input>();
                            match event.state {
                                winit::event::ElementState::Pressed => input.press(code),
                                winit::event::ElementState::Released => input.release(code),
                            }
                        }
                    },
                    _ => (),
                }
            },
            winit::event::Event::DeviceEvent {
                event: winit::event::DeviceEvent::MouseMotion { delta },
                ..
            } => {
                let events = app.state.resource_mut::<Events<WindowEvent>>();
                let delta = Vec2::new(delta.0 as f32, delta.1 as f32);
                events.send(WindowEvent::CursorMove(delta));
            },
            _ => (),
        })
        .unwrap();
}
