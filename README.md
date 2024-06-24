# Ashell

Barely customizable Wayland status bar for Hyprland compositor. 

WIP, highly unstable

### Does it only work on Hyprland?
While it's currently tailored for Hyprland, it could work with other compositors. 

However, at present, it relies on [hyprland-rs](https://github.com/hyprland-community/hyprland-rs) 
to gather information about the active window and workspaces and I haven't implemented any 
feature flags to disable these functionalities or alternative methods to obtain this data.

## Features

- Lancher button
- OS Updates indicator
- Hyprland Active Window
- Hyprland Workspaces
- System Information (CPU, RAM, Temperature)
- Date time
- Settings panel
    - Power menu
    - Battery information
    - Audio sources and sinks
    - Screen brightness
    - Network stuff
    - VPN
    - Bluetooth
    - Power profiles
    - Idle inhibitor

## Configuration
The configuration uses the yaml file format and is named `~/.config/ashell.yml`

``` yaml
# Ashell log level filter, possible values "DEBUG" | "INFO" | "WARNING" | "ERROR"
logLevel: "INFO" # optional, default "INFO"
# App lancher commanda, it will be used to open the launcher,
# without a value the related button will not appear
appLauncherCmd: "~/.config/rofi/launcher.sh" # optional, default None 
# Update module configuration. 
# Without a value the related button will not appear.
updates: # optional, default None 
  # The check command will be used to retrieve the update list.
  # It should return something like `package_name version_from -> version_to\n`
  checkCmd: "checkupdates; paru -Qua" # required
  # The update command is used to init the OS update process
  updateCmd: "alacritty -e bash -c \"paru; echo Done - Press enter to exit; read\" &" # required
# Maximum number of chars that can be present in the window title
# after that the title will be truncated 
truncateTitleAfterLength: 150 # optional, default 150
# The system module configuration
system: 
  disabled: false # Enable or disable the system monitor module
  cpuWarnThreshold: 6O # cpu indicator warning level (default 60)
  cpuAlertThreshold: 8O # cpu indicator alert level (default 80)
  memWarnThreshold: 7O # mem indicator warning level (default 70)
  memAlertThreshold: 85 # mem indicator alert level (default 85)
  tempWarnThreshold: 6O # temperature indicator warning level (default 60)
  tempAlertThreshold: 8O # temperature indicator alert level (default 80)
# Clock module configuration
clock:
  # clock format see: https://docs.rs/chrono/latest/chrono/format/strftime/index.html 
  format: "%a %d %b %R" # optional, default: %a %d %b %R
# Settings module configuration
settings:
  # command used for lock the system
  # without a value the related button will not appear 
  lockCmd: "hyprlock &" # optional, default None 
  # command used to open the sinks audio settings 
  # without a value the related button will not appear 
  audioSinksMoreCmd: "pavucontrol -t 3" # optional default None 
  # command used to open the sources audio settings
  # without a value the related button will not appear 
  audioSourcesMoreCmd: "pavucontrol -t 4" # optional, default None 
  # command used to open the network settings 
  # without a value the related button will not appear 
  wifiMoreCmd: "nm-connection-editor" # optional, default None
  # command used to open the VPN settings 
  # without a value the related button will not appear 
  vpnMoreCmd: "nm-connection-editor" # optional, default None 
  # command used to open the Bluetooth settings  
  # without a value the related button will not appear 
  bluetoothMoreCmd: "blueman-manager" # optional, default None 
```

### So, what's the purpose of this project?
While, I could have used [waybar](https://github.com/Alexays/Waybar) that's for sure is a 
a great project but I wanted something more sophisticated 
with submenus and other stuff.

I tried with other great projects like [eww](https://github.com/elkowar/eww) but
instead of writing or copy-paste eww configurations I prefered to create 
my Wayland bar.

So, using the pop-os fork of [iced](https://github.com/pop-os/iced), I started to 
create this project.

In the end, what can this project do for you? 

Almost nothing but it could be useful if you want to create your own status bar 
or if you have to read some examples on how to get information from `dbus`.

So feel free to fork this project and customize it for your needs.

## Some screenshots

#### Main bar
![MainBar](https://raw.githubusercontent.com/MalpenZibo/ashell/main/screenshots/ashell.png)

#### Updates
![Updates](https://raw.githubusercontent.com/MalpenZibo/ashell/main/screenshots/updates-panel.png)

#### Settings
![Settings](https://raw.githubusercontent.com/MalpenZibo/ashell/main/screenshots/settings-panel.png)

#### Power menu
![PowerMenu](https://raw.githubusercontent.com/MalpenZibo/ashell/main/screenshots/power-menu.png)

#### Pulse Audio
![PulseAudio](https://raw.githubusercontent.com/MalpenZibo/ashell/main/screenshots/sinks-selection.png)

#### Network
![Network](https://raw.githubusercontent.com/MalpenZibo/ashell/main/screenshots/network-menu.png)

#### Bluetooth
![Bluetooth](https://raw.githubusercontent.com/MalpenZibo/ashell/main/screenshots/bluetooth-menu.png)


