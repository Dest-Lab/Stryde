# Config file

### Configuration Overview

When you open the Stryde configuration file for the first time\
`~/.config/stryde/config.toml` you will see something like this

```toml
theme = "Stryde-Dark"
antialiasing = false
font_name = " "
placeholder = "Type commands, search..."
app_width = 774.0
app_height = 500.0
list_text_size = 16
input_text_size = 18
icon_size = 37
padding_vertical = 0.0
spacing = 5
highlight_style_text = false
divider = true
show_apps = true
close_on_launch = true
default_terminal = "kitty"
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

#### **font_name**

Change default font to a font installed on your system (example: "JetBrains Mono")

#### **placeholder**

Sets the placeholder text in search bar

#### **app_width**

The width of the Stryde window in pixels

#### **app_height**

The height of the Stryde window in pixels

#### **list_text_size**

Defines the font size for the list of applications

#### **input_text_size**

Defines the font size used in search bar

#### **icon_size**

The size of icons in the application list

#### **padding_vertical**

Sets the padding at the top and bottom of the application list

#### **spacing**

Defines space between apps in the list

#### **highlight_style_text**

If set to true, the selected app will be highlighted using text color instead of background

#### **divider**

If set to true, shows a divider line between the input field and the app list

#### **show_apps**

Show application when you start the app (if set to false it will show apps only when you search)

#### **close_on_launch**

Close the Stryde after opening an application

#### **default_terminal**

Specifies which terminal to use when opening terminal apps (like btop)
