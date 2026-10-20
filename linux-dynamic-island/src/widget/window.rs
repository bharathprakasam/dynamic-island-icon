use crate::errors::AppResult;
use crate::widget::animation::DimensionInterpolator;
use crate::widget::state::WidgetState;

pub struct IslandWindow {
    pub state: WidgetState,
    pub interpolator: DimensionInterpolator,
    pub compact_w: f64,
    pub compact_h: f64,
    pub expanded_w: f64,
    pub expanded_h: f64,
}

impl IslandWindow {
    pub fn new(compact_w: f64, compact_h: f64, expanded_w: f64, expanded_h: f64) -> Self {
        let initial_radius = compact_h / 2.0;
        let interpolator = DimensionInterpolator::new(compact_w, compact_h, initial_radius, 1.0);

        Self {
            state: WidgetState::Collapsed,
            interpolator,
            compact_w,
            compact_h,
            expanded_w,
            expanded_h,
        }
    }

    pub fn set_state(&mut self, new_state: WidgetState) -> AppResult<()> {
        self.state = new_state;

        match new_state {
            WidgetState::Hidden => {
                self.interpolator.set_target(
                    self.compact_w * 0.8,
                    self.compact_h * 0.5,
                    self.compact_h / 2.0,
                    0.0,
                );
            }
            WidgetState::Collapsed => {
                self.interpolator.set_target(
                    self.compact_w,
                    self.compact_h,
                    self.compact_h / 2.0,
                    1.0,
                );
            }
            WidgetState::Hovered => {
                self.interpolator.set_target(
                    self.compact_w * 1.15,
                    self.compact_h * 1.05,
                    (self.compact_h * 1.05) / 2.0,
                    1.0,
                );
            }
            WidgetState::Expanded => {
                self.interpolator.set_target(
                    self.expanded_w,
                    self.expanded_h,
                    28.0,
                    1.0,
                );
            }
        }

        Ok(())
    }

    pub fn toggle_expanded(&mut self) -> AppResult<()> {
        if self.state == WidgetState::Expanded {
            self.set_state(WidgetState::Collapsed)
        } else {
            self.set_state(WidgetState::Expanded)
        }
    }

    /// Advance visual animation step by `dt` seconds
    pub fn update(&mut self, dt: f64) -> bool {
        self.interpolator.update(dt)
    }

    pub fn current_dimensions(&self) -> (f64, f64, f64, f64) {
        (
            self.interpolator.width.current,
            self.interpolator.height.current,
            self.interpolator.corner_radius.current,
            self.interpolator.opacity.current,
        )
    }
}
