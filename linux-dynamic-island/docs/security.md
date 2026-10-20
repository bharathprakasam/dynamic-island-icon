# Security & Permissions

Linux Dynamic Island interacts with system notifications, global key bindings, and application execution.

## Sandboxing & Flatpak

When packaged as a Flatpak, the application requests the following portal permissions:
- `org.freedesktop.Notifications`: To display notifications and monitor system alerts.
- `org.freedesktop.portal.GlobalShortcuts`: To register global activation hotkeys without raw input capture.
- `wayland` & `fallback-x11`: Socket access to attach overlay surfaces to the compositor.

## Desktop File Execution

The launcher safely verifies application executable paths and does not run arbitrary unvalidated shell scripts or untrusted inputs.
