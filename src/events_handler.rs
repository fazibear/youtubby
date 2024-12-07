use crate::{
    last_fm, platform::platform, player_state, player_state_changed::PlayerStateChanged,
    tray_handler, window_handler, Youtubby,
};
use anyhow::Result;
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use log::debug;
use muda::MenuEvent;
use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconId};

pub fn callback(
    app: &mut Youtubby,
    event: &Event<PlayerStateChanged>,
    control_flow: &mut ControlFlow,
) -> Result<()> {
    platform::set_control_flow(control_flow);

    match event {
        Event::UserEvent(user_event) => match user_event {
            PlayerStateChanged::Ended => {
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
                if app.preferences.always_use_audio {
                    app.window_handler
                        .webview
                        .evaluate_script("Youtubby.switchToAudio()")?;
                }
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
        },
        Event::WindowEvent {
            event: WindowEvent::Focused(false),
            ..
        } if app.preferences.hide_unfocused_window => app.window_handler.hide(),
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => app.window_handler.hide(),
        e => debug!("Event: {e:?}"),
    };

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
            "always_use_audio" => {
                app.preferences.always_use_audio = !app.preferences.always_use_audio;
                app.preferences.save()?;
                tray_handler::refresh(app)?;
            }
            e => debug!("MenuEvent: {e:?}"),
        }
    }

    Ok(())
}

fn exit(control_flow: &mut ControlFlow, app: &Youtubby) -> Result<()> {
    app.preferences.save()?;
    *control_flow = ControlFlow::Exit;
    Ok(())
}
