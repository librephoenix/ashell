use std::ops::Deref;

use super::{Module, OnModulePress};
use crate::{
    app,
    components::icons::{icon, Icons},
    config::MediaPlayerModuleConfig,
    menu::MenuType,
    services::{
        mpris::{MprisPlayerCommand, MprisPlayerService, PlayerCommand},
        ReadOnlyService, Service, ServiceEvent,
    },
    style::SettingsButtonStyle,
    utils::truncate_text,
};
use iced::{
    widget::{button, column, container, row, slider, text},
    Alignment::{self, Center},
    Element, Subscription, Task, Theme,
};

#[derive(Default)]
pub struct MediaPlayer {
    data: Vec<PlayerData>,
    service: Option<MprisPlayerService>,
}

struct PlayerData {
    name: String,
    song: Option<String>,
    volume: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Prev(String),
    PlayPause(String),
    Next(String),
    SetVolume(String, f64),
    Event(ServiceEvent<MprisPlayerService>),
}

impl MediaPlayer {
    pub fn update(
        &mut self,
        message: Message,
        config: &MediaPlayerModuleConfig,
    ) -> Task<crate::app::Message> {
        match message {
            Message::Prev(n) => {
                if let Some(s) = self.service.as_mut() {
                    s.command(MprisPlayerCommand {
                        service: n,
                        command: PlayerCommand::Prev,
                    })
                    .map(|event| crate::app::Message::MediaPlayer(Message::Event(event)))
                } else {
                    Task::none()
                }
            }
            Message::PlayPause(n) => {
                if let Some(s) = self.service.as_mut() {
                    s.command(MprisPlayerCommand {
                        service: n,
                        command: PlayerCommand::PlayPause,
                    })
                    .map(|event| crate::app::Message::MediaPlayer(Message::Event(event)))
                } else {
                    Task::none()
                }
            }
            Message::Next(n) => {
                if let Some(s) = self.service.as_mut() {
                    s.command(MprisPlayerCommand {
                        service: n,
                        command: PlayerCommand::Next,
                    })
                    .map(|event| crate::app::Message::MediaPlayer(Message::Event(event)))
                } else {
                    Task::none()
                }
            }
            Message::SetVolume(n, v) => {
                if let Some(s) = self.service.as_mut() {
                    s.command(MprisPlayerCommand {
                        service: n,
                        command: PlayerCommand::Volume(v),
                    })
                    .map(|event| crate::app::Message::MediaPlayer(Message::Event(event)))
                } else {
                    Task::none()
                }
            }
            Message::Event(d) => match d {
                ServiceEvent::Init(s) => {
                    let data = s.deref();
                    self.data = data
                        .iter()
                        .map(|d| PlayerData {
                            name: d.service.clone(),
                            song: d.metadata.clone().and_then(|d| match (d.artists, d.title) {
                                (None, None) => None,
                                (None, Some(t)) => Some(truncate_text(&t, config.max_title_length)),
                                (Some(a), None) => {
                                    Some(truncate_text(&a.join(", "), config.max_title_length))
                                }
                                (Some(a), Some(t)) => Some(truncate_text(
                                    &format!("{} - {}", a.join(", "), t),
                                    config.max_title_length,
                                )),
                            }),
                            volume: d.volume,
                        })
                        .collect();
                    self.service = Some(s);
                    Task::none()
                }
                ServiceEvent::Update(d) => {
                    self.data = d
                        .iter()
                        .map(|d| PlayerData {
                            name: d.service.clone(),
                            song: d.metadata.clone().and_then(|d| match (d.artists, d.title) {
                                (None, None) => None,
                                (None, Some(t)) => Some(t),
                                (Some(a), None) => Some(a.join(", ")),
                                (Some(a), Some(t)) => Some(format!("{} - {}", a.join(", "), t)),
                            }),
                            volume: d.volume,
                        })
                        .collect();
                    Task::none()
                }
                ServiceEvent::Error(_) => Task::none(),
            },
        }
    }

    pub fn menu_view(&self) -> Element<Message> {
        column(self.data.iter().map(|d| {
            container(
                column![]
                    .push_maybe(d.song.clone().map(|s| text(s)))
                    .push_maybe(d.volume.map(|v| {
                        slider(0.0..=100.0, v, |new_v| {
                            Message::SetVolume(d.name.clone(), new_v)
                        })
                    }))
                    .push(
                        row![
                            button(icon(Icons::SkipPrevious))
                                .on_press(Message::Prev(d.name.clone()))
                                .padding([5, 12])
                                .style(SettingsButtonStyle.into_style()),
                            button(icon(Icons::PlayPause))
                                .on_press(Message::PlayPause(d.name.clone()))
                                .style(SettingsButtonStyle.into_style()),
                            button(icon(Icons::SkipNext))
                                .on_press(Message::Next(d.name.clone()))
                                .padding([5, 12])
                                .style(SettingsButtonStyle.into_style())
                        ]
                        .spacing(8),
                    )
                    .spacing(8)
                    .align_x(Center),
            )
            .padding(4)
            .style(|theme: &Theme| container::Style {
                background: Some(iced::Background::Color(theme.palette().primary)),
                ..container::Style::default()
            })
            .into()
        }))
        .spacing(8)
        .align_x(Alignment::Center)
        .into()
    }
}

impl Module for MediaPlayer {
    type ViewData<'a> = ();
    type SubscriptionData<'a> = ();

    fn view(
        &self,
        (): Self::ViewData<'_>,
    ) -> Option<(Element<app::Message>, Option<OnModulePress>)> {
        Some((
            text("media").into(),
            Some(OnModulePress::ToggleMenu(MenuType::MediaPlayer)),
        ))
        // self.data[0].song.clone().map(|s| {
        //     (
        //         text(s).size(12).into(),
        //         Some(OnModulePress::ToggleMenu(MenuType::MediaPlayer)),
        //     )
        // })
    }

    fn subscription(&self, (): Self::SubscriptionData<'_>) -> Option<Subscription<app::Message>> {
        Some(
            MprisPlayerService::subscribe()
                .map(|event| app::Message::MediaPlayer(Message::Event(event))),
        )
    }
}
