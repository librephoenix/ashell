use crate::{
    config::AppearanceColor,
    style::{header_pills, WorkspaceButtonStyle},
};
use hyprland::{
    event_listener::AsyncEventListener,
    shared::{HyprData, HyprDataActive, HyprDataVec},
};
use iced::{
    alignment,
    subscription::channel,
    theme::Button,
    widget::{button, container, text, Row},
    Element, Length, Subscription,
};
use log::error;
use std::{
    any::TypeId,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub monitor_id: Option<usize>,
    pub active: bool,
    pub windows: u16,
}

fn get_workspaces() -> Vec<Workspace> {
    let active = hyprland::data::Workspace::get_active().unwrap();
    let mut workspaces = hyprland::data::Workspaces::get()
        .map(|w| w.to_vec())
        .unwrap_or_default();

    workspaces.sort_by_key(|w| w.id);

    let mut current: usize = 1;

    workspaces
        .into_iter()
        .flat_map(|w| {
            if w.id < 0 {
                vec![Workspace {
                    id: w.id,
                    name: w
                        .name
                        .split(":")
                        .last()
                        .map_or_else(|| "".to_string(), |s| s.to_owned()),
                    monitor_id: Some(w.monitor_id as usize),
                    active: w.id == active.id,
                    windows: w.windows,
                }]
            } else {
                let missing: usize = w.id as usize - current;
                let mut res = Vec::with_capacity(missing + 1);
                for i in 0..missing {
                    res.push(Workspace {
                        id: (current + i) as i32,
                        name: (current + i).to_string(),
                        monitor_id: None,
                        active: false,
                        windows: 0,
                    });
                }
                current += missing + 1;
                res.push(Workspace {
                    id: w.id,
                    name: w.name.clone(),
                    monitor_id: Some(w.monitor_id as usize),
                    active: w.id == active.id,
                    windows: w.windows,
                });

                res
            }
        })
        .collect::<Vec<Workspace>>()
}

pub struct Workspaces {
    workspaces: Vec<Workspace>,
}

impl Default for Workspaces {
    fn default() -> Self {
        Self {
            workspaces: get_workspaces(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    WorkspacesChanged(Vec<Workspace>),
    ChangeWorkspace(i32),
}

impl Workspaces {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::WorkspacesChanged(workspaces) => {
                self.workspaces = workspaces;
            }
            Message::ChangeWorkspace(id) => {
                let active_workspace = self.workspaces.iter().find(|w| w.active);

                if active_workspace.is_none() || active_workspace.map(|w| w.id) != Some(id) {
                    let res = hyprland::dispatch::Dispatch::call(
                        hyprland::dispatch::DispatchType::Workspace(
                            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(id),
                        ),
                    );

                    if let Err(e) = res {
                        error!("failed to dispatch workspace change: {:?}", e);
                    }
                }
            }
        }
    }

    pub fn view(
        &self,
        workspace_colors: &[AppearanceColor],
        special_workspace_colors: Option<&[AppearanceColor]>,
    ) -> Element<Message> {
        container(
            Row::with_children(
                self.workspaces
                    .iter()
                    .map(|w| {
                        let empty = w.windows == 0;
                        let monitor = w.monitor_id;

                        let color = monitor.map(|m| {
                            if w.id > 0 {
                                workspace_colors.get(m).copied()
                            } else {
                                special_workspace_colors
                                    .unwrap_or(workspace_colors)
                                    .get(m)
                                    .copied()
                            }
                        });

                        button(
                            container(
                                if w.id < 0 {
                                    text(w.name.as_str())
                                } else {
                                    text(w.id)
                                }
                                .size(10),
                            )
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center),
                        )
                        .style(Button::custom(WorkspaceButtonStyle(empty, color)))
                        .padding(if w.id < 0 {
                            if w.active {
                                [0, 16]
                            } else {
                                [0, 8]
                            }
                        } else {
                            [0, 0]
                        })
                        .on_press(Message::ChangeWorkspace(w.id))
                        .width(if w.id < 0 {
                            Length::Shrink
                        } else if w.active {
                            Length::Fixed(32.)
                        } else {
                            Length::Fixed(16.)
                        })
                        .height(16)
                        .into()
                    })
                    .collect::<Vec<Element<'_, _, _>>>(),
            )
            .spacing(4),
        )
        .padding([4, 8])
        .align_y(alignment::Vertical::Center)
        .height(Length::Shrink)
        .style(header_pills)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let id = TypeId::of::<Self>();

        channel(id, 10, |output| async move {
            let output = Arc::new(RwLock::new(output));
            loop {
                let mut event_listener = AsyncEventListener::new();

                event_listener.add_workspace_added_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(Message::WorkspacesChanged(get_workspaces()))
                                    .expect("error getting workspaces: workspace added event");
                            }
                        })
                    }
                });

                event_listener.add_workspace_changed_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(Message::WorkspacesChanged(get_workspaces()))
                                    .expect("error getting workspaces: workspace change event");
                            }
                        })
                    }
                });

                event_listener.add_workspace_deleted_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(Message::WorkspacesChanged(get_workspaces()))
                                    .expect("error getting workspaces: workspace destroy event");
                            }
                        })
                    }
                });

                event_listener.add_workspace_moved_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(Message::WorkspacesChanged(get_workspaces()))
                                    .expect("error getting workspaces: workspace moved event");
                            }
                        })
                    }
                });

                event_listener.add_window_closed_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(Message::WorkspacesChanged(get_workspaces()))
                                    .expect("error getting workspaces: window close event");
                            }
                        })
                    }
                });

                event_listener.add_window_opened_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(Message::WorkspacesChanged(get_workspaces()))
                                    .expect("error getting workspaces: window open event");
                            }
                        })
                    }
                });

                event_listener.add_window_moved_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(Message::WorkspacesChanged(get_workspaces()))
                                    .expect("error getting workspaces: window moved event");
                            }
                        })
                    }
                });

                event_listener.add_active_monitor_changed_handler({
                    let output = output.clone();
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            if let Ok(mut output) = output.write() {
                                output
                                    .try_send(Message::WorkspacesChanged(get_workspaces()))
                                    .expect(
                                        "error getting workspaces: active monitor change event",
                                    );
                            }
                        })
                    }
                });

                let res = event_listener.start_listener_async().await;

                if let Err(e) = res {
                    error!("restarting workspaces listener due to error: {:?}", e);
                }
            }
        })
    }
}
