use super::{
    last_fm, player_state, player_state_changed::PlayerStateChanged, tray_handler, window_handler,
    Youtubby,
};
use anyhow::Result;
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use log::debug;
use muda::MenuEvent;
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconId};
use winit::{event::WindowEvent, event_loop::ControlFlow};

pub fn handle_window_events(
    app: &mut Youtubby,
    event: &WindowEvent,
    control_flow: &mut ControlFlow,
) -> Result<()> {
    match event {
        WindowEvent::Focused(false) if app.preferences.hide_unfocused_window => {
            app.window_handler.hide()
        }
        WindowEvent::CloseRequested => exit(control_flow, app)?,
        e => debug!("Event: {e:?}"),
    };
    Ok(())
}

pub fn handle_user_events(
    app: &mut Youtubby,
    event: &PlayerStateChanged,
    _control_flow: &mut ControlFlow,
) -> Result<()> {
    match event {
        PlayerStateChanged::Stop => {
            app.player_state.state = player_state::State::Stoped;
            tray_handler::refresh(app)?;
        }
        PlayerStateChanged::Pause => {
            app.player_state.state = player_state::State::Paused;
            tray_handler::refresh(app)?;
        }
        PlayerStateChanged::Emptied => {
            app.player_state.reset();
            tray_handler::refresh(app)?;
        }
        PlayerStateChanged::Play => {
            app.player_state.state = player_state::State::Playing;
            tray_handler::refresh(app)?;
        }
        PlayerStateChanged::LoadMetaData(metadata) => {
            app.player_state.metadata = metadata.clone();
            last_fm::track_update_now_playing(app)?;
            tray_handler::refresh(app)?;
        }
        PlayerStateChanged::MetaDataUpdate(metadata) => {
            app.player_state.metadata = metadata.clone();
            tray_handler::refresh(app)?;
        }
        PlayerStateChanged::TimeUpdate(time) => {
            app.player_state.position = Some(*time);
            last_fm::track_scrobble_at_half(app)?;
        }
        PlayerStateChanged::DurationChange(duration) => {
            app.player_state.duration = Some(*duration);
        }
        e => log::debug!("Unhandled PlayerState Event: {e:?}"),
    }
    Ok(())
}

pub fn handle_hotkey_events(app: &mut Youtubby, _control_flow: &mut ControlFlow) -> Result<()> {
    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        match event {
            GlobalHotKeyEvent {
                id,
                state: HotKeyState::Pressed,
                ..
            } => {
                if let Some(&js) = app.key_handler.keys.get(&id) {
                    app.window_handler.webview.evaluate_script(js)?
                }
            }
            e => debug!("GlobalHotKeyEvent: {e:?}"),
        }
    }
    Ok(())
}

pub fn handle_tray_events(app: &mut Youtubby, _control_flow: &mut ControlFlow) -> Result<()> {
    if let Ok(event) = TrayIconEvent::receiver().try_recv() {
        match event {
            TrayIconEvent::Click {
                id: TrayIconId(id),
                button: MouseButton::Left,
                button_state: MouseButtonState::Down,
                rect,
                ..
            } if id == "0" => app.window_handler.show_hide(rect.position),
            e => debug!("TrayIconEvent: {e:?}"),
        }
    }
    Ok(())
}

pub fn handle_menu_events(app: &mut Youtubby, control_flow: &mut ControlFlow) -> Result<()> {
    if let Ok(event) = MenuEvent::receiver().try_recv() {
        match event.id.0.as_str() {
            "show" => app.window_handler.show(),
            "playstop" => app
                .window_handler
                .webview
                .evaluate_script("Youtubby.playPauseClick()")?,
            "next" => app.window_handler.webview.evaluate_script("")?,
            "prev" => app.window_handler.webview.evaluate_script("")?,
            "quit" => exit(control_flow, app)?,
            "always_on_top" => {
                app.preferences.always_on_top = !app.preferences.always_on_top;
                app.preferences.save()?;
                window_handler::refresh(app)?;
            }
            "hide_unfocused_window" => {
                app.preferences.hide_unfocused_window = !app.preferences.hide_unfocused_window;
                app.preferences.save()?;
            }
            "show_info_in_tray" => {
                app.preferences.show_info_in_tray = !app.preferences.show_info_in_tray;
                tray_handler::refresh(app)?;
            }
            "show_info_in_tooltip" => {
                app.preferences.show_info_in_tooltip = !app.preferences.show_info_in_tooltip;
                app.preferences.save()?;
                tray_handler::refresh(app)?;
            }
            "last_fm_action" => {
                last_fm::menu_click(app)?;
                last_fm::set_menu(app);
                app.preferences.save()?;
            }
            e => debug!("MenuEvent: {e:?}"),
        }
    }
    Ok(())
}

fn exit(_control_flow: &mut ControlFlow, app: &Youtubby) -> Result<()> {
    app.preferences.save()?;
    //exit!
    Ok(())
}
