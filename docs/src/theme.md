# Theme

### Creating a Custom Theme

To create a custom theme, you need to place it in the `~/.config/stryde/themes` folder and create a file named:

```
your_theme_name.toml
```

#### Theme File Structure

```toml
background = ""
text = ""
primary = ""
secondary = ""
selected = ""
```

* All colors must be **hex values (e.g** #000000)
* Using `#` is optional

#### Parameters

* **background** – background color of the app
* **text** – text color (used for list items and search bar)
* **primary** – primary color (used for selected text)
* **secondary** – secondary color (used for placeholders in search bar and divider line)
* **selected** – color of the currently selected app when navigating with arrows
