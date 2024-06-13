use crate::app::App;
use crate::player_state_changed::PlayerStateChanged;
use crate::{last_fm, player_state, tray_handler};
use anyhow::Result;
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use log::debug;
use muda::MenuEvent;
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
use tray_icon::{MouseButton, MouseButtonState, TrayIconEvent, TrayIconId};

pub fn callback(
    app: &mut App,
    event: &Event<PlayerStateChanged>,
    control_flow: &mut ControlFlow,
) -> Result<()> {
    *control_flow = ControlFlow::Poll;

    match event {
        Event::UserEvent(user_event) => {
            log::info!("{:?}", user_event);

            match user_event {
                PlayerStateChanged::Stop => app.player_state.state = player_state::State::Stoped,
                PlayerStateChanged::Pause => app.player_state.state = player_state::State::Paused,
                PlayerStateChanged::Emptied => app.player_state.reset(),
                PlayerStateChanged::Play(metadata) => {
                    app.player_state.metadata = metadata.clone();
                    app.player_state.state = player_state::State::Playing;
                    last_fm::track_update_now_playing(app)?;
                }

                PlayerStateChanged::MetaDataUpdate(metadata) => {
                    app.player_state.metadata = metadata.clone();
                }
                PlayerStateChanged::TimeUpdate(time) => {
                    app.player_state.position = Some(*time);

                    if let (Some(duration), Some(position)) =
                        (app.player_state.duration, app.player_state.position)
                    {
                        if duration / 2 == position as i64 {
                            last_fm::track_scrobble(app)?;
                        }
                    }
                }
                PlayerStateChanged::DurationChange(duration) => {
                    app.player_state.duration = Some(*duration);
                }
                e => log::debug!("PlayerState: {e:?}"),
            }
            tray_handler::refresh(app)?;
        }
        Event::WindowEvent {
            event: WindowEvent::Focused(false),
            ..
        } if app.preferences.hide_unfocused_window => app.window_handler.hide(),
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => exit(control_flow, app)?,
        Event::MainEventsCleared => {
            app.window_handler.window.request_redraw();
        }
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
            "hide_unfocused_window" => {
                app.preferences.hide_unfocused_window = !app.preferences.hide_unfocused_window;
                app.preferences.save()?;
                tray_handler::refresh(app)?;
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

fn exit(control_flow: &mut ControlFlow, app: &App) -> Result<()> {
    app.preferences.save()?;
    *control_flow = ControlFlow::Exit;
    Ok(())
}
