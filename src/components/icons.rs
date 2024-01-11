use iced::{
    widget::{text, Text},
    Font,
};

#[derive(Copy, Clone, Default)]
pub enum Icons {
    #[default]
    None,
    Launcher,
    Refresh,
    NoUpdatesAvailable,
    UpdatesAvailable,
    MenuClosed,
    MenuOpen,
    Cpu,
    Mem,
    Temp,
    Speaker0,
    Speaker1,
    Speaker2,
    Speaker3,
    Headphones0,
    Headphones1,
    Headset,
    Mic0,
    Mic1,
    MonitorSpeaker,
    Battery0,
    Battery1,
    Battery2,
    Battery3,
    Battery4,
    BatteryCharging,
    Wifi0,
    Wifi1,
    Wifi2,
    Wifi3,
    Wifi4,
    Ethernet,
    Vpn,
    Lock,
    Power,
    RightArrow,
    Brightness,
    Point,
    Close,
}

impl From<Icons> for &'static str {
    fn from(icon: Icons) -> &'static str {
        match icon {
            Icons::None => "",
            Icons::Launcher => "󱗼",
            Icons::Refresh => "󰑐",
            Icons::NoUpdatesAvailable => "󰗠",
            Icons::UpdatesAvailable => "󰳛",
            Icons::MenuClosed => "",
            Icons::MenuOpen => "",
            Icons::Cpu => "󰔂",
            Icons::Mem => "󰘚",
            Icons::Temp => "󰔏",
            Icons::Speaker0 => "󰸈",
            Icons::Speaker1 => "󰕿",
            Icons::Speaker2 => "󰖀",
            Icons::Speaker3 => "󰕾",
            Icons::Headphones0 => "󰟎",
            Icons::Headphones1 => "󰋋",
            Icons::Headset => "󰋎",
            Icons::Mic0 => "󰍭",
            Icons::Mic1 => "󰍬",
            Icons::MonitorSpeaker => "󰽟",
            Icons::Battery0 => "󰂃",
            Icons::Battery1 => "󰁼",
            Icons::Battery2 => "󰁾",
            Icons::Battery3 => "󰂀",
            Icons::Battery4 => "󰁹",
            Icons::BatteryCharging => "󰂄",
            Icons::Wifi0 => "󰤭",
            Icons::Wifi1 => "󰤟",
            Icons::Wifi2 => "󰤢",
            Icons::Wifi3 => "󰤥",
            Icons::Wifi4 => "󰤨",
            Icons::Ethernet => "󰈀",
            Icons::Vpn => "󰖂",
            Icons::Lock => "",
            Icons::Power => "󰐥",
            Icons::RightArrow => "󰁔",
            Icons::Brightness => "󰃟",
            Icons::Point => "",
            Icons::Close => "󰅖",
        }
    }
}

pub fn icon<'a>(r#type: Icons) -> Text<'a> {
    text(std::convert::Into::<&'static str>::into(r#type))
        .font(Font::with_name("Symbols Nerd Font Mono"))
        .size(12)
}
