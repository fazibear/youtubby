use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use tao::event::{Event, WindowEvent};
use tao::event_loop::ControlFlow;
use tray_icon::{ClickType, TrayIconEvent, TrayIconId};

use crate::key_handler::KeyHandler;
use crate::menu_handler::MenuHandler;
use crate::tray_handler::TrayHandler;
use crate::window_handler::{UserEvent, WindowHandler};

pub struct EventsHandler {}

impl EventsHandler {
    pub fn callback(
        event: &Event<UserEvent>,
        control_flow: &mut ControlFlow,
        window_handler: &mut WindowHandler,
        key_handler: &mut KeyHandler,
        menu_handler: &mut MenuHandler,
        tray_handler: &mut TrayHandler,
    ) {
        *control_flow = ControlFlow::Wait;

        if let Event::UserEvent(UserEvent::PlayerStateUpdated(meta)) = event {
            let play = if meta.state == "playing" {
                "▶"
            } else {
                "⏸"
            };
            let info = format!("{} {} - {}", play, meta.artist, meta.title);
            tray_handler.icon.set_title(Some(info.clone()));
            tray_handler.icon.set_tooltip(Some(info)).unwrap();
        };

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Focused(false) => window_handler.window.set_visible(false),

                _ => {}
            }
        }

        if let Ok(GlobalHotKeyEvent {
            id,
            state: HotKeyState::Pressed,
        }) = key_handler.channel.try_recv()
        {
            if let Some(&js) = key_handler.keys.get(&id) {
                window_handler.webview.evaluate_script(js).unwrap();
            }
        }

        if let Ok(TrayIconEvent {
            id: TrayIconId(id),
            click_type: ClickType::Left,
            icon_rect,
            ..
        }) = tray_handler.channel.try_recv()
        {
            if &id == "0" {
                window_handler.show_hide(icon_rect.position);
            }
        }

        if let Ok(event) = menu_handler.channel.try_recv() {
            if event.id == "quit" {
                *control_flow = ControlFlow::Exit;
            }
        }
    }
}
