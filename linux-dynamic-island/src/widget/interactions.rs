#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionEvent {
    MouseEnter,
    MouseLeave,
    Click,
    DoubleClick,
    SwipeDown,
    SwipeUp,
}

pub struct InteractionHandler;

impl InteractionHandler {
    pub fn handle(event: InteractionEvent) {
        match event {
            InteractionEvent::MouseEnter => log::debug!("Island hovered"),
            InteractionEvent::MouseLeave => log::debug!("Island unhovered"),
            InteractionEvent::Click => log::debug!("Island clicked"),
            InteractionEvent::DoubleClick => log::debug!("Island double clicked"),
            InteractionEvent::SwipeDown => log::debug!("Island swipe down"),
            InteractionEvent::SwipeUp => log::debug!("Island swipe up"),
        }
    }
}
