use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconId};

use crate::key_handler::KeyHandler;
use crate::last_fm::LastFm;
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
        last_fm: &mut LastFm,
        state: &mut State,
    ) {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::PlayerStateUpdated(meta)) => {
                state.update_player_info(meta);
                tray_handler.refresh(state);
            }
            Event::WindowEvent {
                event: WindowEvent::Focused(false),
                ..
            } if state.preferences.hide_unfocused_window => window_handler.hide(),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => Self::exit(control_flow, state),
            _e => {} //println!("{:?}", e),
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
                _e => {} // println!("{:?}", e),
            }
        }

        if let Ok(event) = menu_handler.channel.try_recv() {
            match event.id.0.as_str() {
                "show" => window_handler.show(),
                "playstop" => window_handler
                    .webview
                    .evaluate_script("PlayPauseClick()")
                    .unwrap(),
                "next" => window_handler.webview.evaluate_script("").unwrap(),
                "prev" => window_handler.webview.evaluate_script("").unwrap(),
                "quit" => Self::exit(control_flow, state),
                "hide_unfocused_window" => {
                    state.preferences.hide_unfocused_window =
                        !state.preferences.hide_unfocused_window;
                    state.preferences.save();
                    tray_handler.refresh(state);
                }
                "show_info_in_tray" => {
                    state.preferences.show_info_in_tray = !state.preferences.show_info_in_tray;
                    state.preferences.save();
                    tray_handler.refresh(state);
                }
                "show_info_in_tooltip" => {
                    state.preferences.show_info_in_tooltip =
                        !state.preferences.show_info_in_tooltip;
                    state.preferences.save();
                    tray_handler.refresh(state);
                }
                "lastfm_auth" => {
                    last_fm.menu_click(state, window_handler, &mut menu_handler.last_fm)
                }
                _e => {} // println!("{:?}", e),
            }
        }
    }

    fn exit(control_flow: &mut ControlFlow, state: &State) {
        state.preferences.save();
        *control_flow = ControlFlow::Exit;
    }
}
