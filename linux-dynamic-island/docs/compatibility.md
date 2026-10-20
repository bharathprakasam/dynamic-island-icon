# Compatibility & Desktop Support

Linux Dynamic Island aims to run cleanly across modern desktop environments and display servers.

## Compositor & Window Manager Support

| Environment | Protocol | Status | Notes |
| ----------- | -------- | ------ | ----- |
| GNOME Shell | Wayland / X11 | Supported | Wayland requires standard layer-shell or portal positioning |
| KDE Plasma | Wayland / X11 | Supported | Native layer-shell integration |
| Hyprland | Wayland | Supported | Fully supported via `wlr-layer-shell` |
| Sway | Wayland | Supported | Fully supported via `wlr-layer-shell` |
| XFCE / i3 / bspwm | X11 | Supported | Uses `_NET_WM_WINDOW_TYPE_DOCK` / strut reservations |

## Dependencies

- GTK 4.10+
- `gtk4-layer-shell` (Wayland setups)
- `dbus` / `libsystemd`
