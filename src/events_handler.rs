use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use muda::{MenuEvent, MenuId};
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconId};

use crate::key_handler::KeyHandler;
use crate::menu_handler::MenuHandler;
use crate::state::State;
use crate::tray_handler::TrayHandler;
use crate::window_handler::{UserEvent, WindowHandler};

pub struct EventsHandler();

impl EventsHandler {
    pub fn callback(
        event: &Event<UserEvent>,
        control_flow: &mut ControlFlow,
        window_handler: &mut WindowHandler,
        key_handler: &mut KeyHandler,
        menu_handler: &mut MenuHandler,
        tray_handler: &mut TrayHandler,
        state: &mut State,
    ) {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::PlayerStateUpdated(meta)) if state.show_song_in_tray => tray_handler.set_title(meta),
            Event::UserEvent(UserEvent::PlayerStateUpdated(meta)) if state.show_song_in_tooltip => tray_handler.set_tooltip(meta),
            Event::WindowEvent { event:  WindowEvent::Focused(false), ..}  if state.hide_unfocused_window => window_handler.hide(),
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
            e => {} //println!("{:?}", e),
        };

        if let Ok(event) = key_handler.channel.try_recv() {
            match event {
                GlobalHotKeyEvent {
                    id,
                    state: HotKeyState::Pressed,
                    ..
                } => {
                    if let Some(&js) = key_handler.keys.get(&id) {
                        window_handler.webview.evaluate_script(js).unwrap()
                    }
                }
                e => println!("{:?}", e),
            }
        }

        if let Ok(event) = tray_handler.channel.try_recv() {
            match event {
                TrayIconEvent::Click {
                    id: TrayIconId(id),
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Down,
                    rect,
                    ..
                } if id == "0" => window_handler.show_hide(rect.position),
                _e =>{} // println!("{:?}", e),
            }
        }

        if let Ok(event) = menu_handler.channel.try_recv() {
            match event.id.0.as_str() {
                "show" => window_handler.show(),
                "quit" => *control_flow = ControlFlow::Exit,
                _e =>{} // println!("{:?}", e),
            }
        }
    }
}
