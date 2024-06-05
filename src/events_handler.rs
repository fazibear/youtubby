use crate::app::App;
use crate::window_handler::UserEvent;
use crate::{last_fm, tray_handler};
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use muda::MenuEvent;
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconId};

pub fn callback(app: &mut App, event: &Event<UserEvent>, control_flow: &mut ControlFlow) {
    *control_flow = ControlFlow::Poll;

    match event {
        Event::UserEvent(UserEvent::PlayerStateUpdated(state)) => {
            println!("{:?}", state);
            app.player_state.update(state);
            last_fm::track_update_now_playing(app);
            last_fm::track_scrobble(app);
            tray_handler::refresh(app);
        }
        Event::WindowEvent {
            event: WindowEvent::Focused(false),
            ..
        } if app.preferences.hide_unfocused_window => app.window_handler.hide(),
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => exit(control_flow, app),
        Event::MainEventsCleared => {
            app.window_handler.window.request_redraw();
        }
        _e => {} //println!("{:?}", e),
    };

    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        match event {
            GlobalHotKeyEvent {
                id,
                state: HotKeyState::Pressed,
                ..
            } => {
                if let Some(&js) = app.key_handler.keys.get(&id) {
                    app.window_handler.webview.evaluate_script(js).unwrap()
                }
            }
            _e => {} //println!("{:?}", e),
        }
    }

    if let Ok(event) = TrayIconEvent::receiver().try_recv() {
        match event {
            TrayIconEvent::Click {
                id: TrayIconId(id),
                button: MouseButton::Left,
                button_state: MouseButtonState::Down,
                rect,
                ..
            } if id == "0" => app.window_handler.show_hide(rect.position),
            _e => {} // println!("{:?}", e),
        }
    }
    if let Ok(event) = MenuEvent::receiver().try_recv() {
        match event.id.0.as_str() {
            "show" => app.window_handler.show(),
            "playstop" => app
                .window_handler
                .webview
                .evaluate_script("PlayPauseClick()")
                .unwrap(),
            "next" => app.window_handler.webview.evaluate_script("").unwrap(),
            "prev" => app.window_handler.webview.evaluate_script("").unwrap(),
            "quit" => exit(control_flow, app),
            "hide_unfocused_window" => {
                app.preferences.hide_unfocused_window = !app.preferences.hide_unfocused_window;
                app.preferences.save();
                tray_handler::refresh(app);
            }
            "show_info_in_tray" => {
                app.preferences.show_info_in_tray = !app.preferences.show_info_in_tray;
                tray_handler::refresh(app);
            }
            "show_info_in_tooltip" => {
                app.preferences.show_info_in_tooltip = !app.preferences.show_info_in_tooltip;
                app.preferences.save();
                tray_handler::refresh(app);
            }
            "last_fm_action" => {
                last_fm::menu_click(app);
                last_fm::set_menu(app);
            }
            _e => {} // println!("{:?}", e),
        }
    }
}

fn exit(control_flow: &mut ControlFlow, app: &App) {
    app.preferences.save();
    *control_flow = ControlFlow::Exit;
}
