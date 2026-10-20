# Architecture Overview

`linux-dynamic-island` is built as an event-driven system layer overlay.

## Key Layers

1. **State & Core (`src/state.rs`, `src/app.rs`)**:
   - Manages top-level island states (Compact, Hovered, Expanded, Notification, Media).
   - Coordinates event loops and cross-subsystem interactions.

2. **Platform & Windowing (`src/platform/`)**:
   - Abstracts Wayland (`gtk4-layer-shell`, `wlr-layer-shell`) and X11 window positioning.
   - Handles global shortcuts via FreeDesktop Portals or X11 key grabs.

3. **Widget & UI (`src/widget/`, `src/ui/`)**:
   - Renders the responsive capsule and expanded panels.
   - Handles CSS theming, animations, easing curves, and user interactions.

4. **Desktop Integration (`src/applications/`)**:
   - Scans XDG standard application directories (`/usr/share/applications`, `~/.local/share/applications`).
   - Parses `.desktop` files and resolves Freedesktop icons.

5. **Services & Event Bus (`src/services/`)**:
   - Listens to DBus notifications (`org.freedesktop.Notifications`).
   - Tracks MPRIS2 media player status updates.
