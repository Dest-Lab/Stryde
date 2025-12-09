# Config file

### Configuration Overview

When you open the Stryde configuration file for the first time\
`~/.config/stryde/config.toml` you will see something like this

```toml
theme = "Stryde-Dark"
antialiasing = false

[window]
width = 774
height = 500

[text]
font_name = " "
list_text_size = 16
input_text_size = 18
placeholder = "Type commands, search..."

[layout]
icon_size = 37
padding_vertical = 0.0
spacing = 5
divider = true

[behavior]
show_apps = true
close_on_launch = true
highlight_style_text = false
default_terminal = "kitty"

[keybinds]
close = "escape"
open = "enter"
navigation = ["arrowup", "arrowdown"]
```

#### **theme**

Specifies which theme file Stryde should load
`Stryde-Dark` is the default dark theme

To use your own custom theme, create a toml file in the `themes` folder
and replace the value with your filename:

```toml
theme = "your_theme_name.toml"
```

#### **antialiasing**

Enables smoothing for text and UI elements\
Turning it on may improve visual quality but can reduce performance

## `[window]`

#### **width**

The width of the Stryde window in pixels

#### **height**

The height of the Stryde window in pixels

## `[text]`

#### **font_name**

Change default font to a font installed on your system (example: "JetBrains Mono")

#### **list_text_size**

Defines the font size for the list of applications

#### **input_text_size**

Defines the font size used in search bar

#### **placeholder**

Sets the placeholder text in search bar

## `[layout]`

#### **icon_size**

The size of icons in the application list

#### **padding_vertical**

Sets the padding at the top and bottom of the application list

#### **spacing**

Defines space between apps in the list

#### **divider**

If set to true, shows a divider line between the input field and the app list

## `[behavior]`

#### **show_apps**

Show application when you start the app (if set to false it will show apps only when you search)

#### **close_on_launch**

Close the Stryde after opening an application

#### **highlight_style_text**

If set to true, the selected app will be highlighted using text color instead of background

#### **default_terminal**

Specifies which terminal to use when opening terminal apps (like btop)

## `[keybinds]`

#### **close**

The key used to close the app

#### **open**

The key used to open the selected app

#### **navigation**

Keys used to navigate the app list: the first key moves up, the second moves down (default: ["arrowup", "arrowdown"])
