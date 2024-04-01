use iced::{border::Radius, theme, widget::{self, shader::wgpu::naga::back}, Border, Theme};


pub struct ThickRule;


impl iced::widget::rule::StyleSheet for ThickRule {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> widget::rule::Appearance {
      let extended_palette = style.extended_palette();
      let color = extended_palette.background.base.text;
      
      widget::rule::Appearance {
          width: 2,
          fill_mode: widget::rule::FillMode::Full,
          color,
          radius: Radius::default(),
      }
    }
}

pub struct ActiveTab;

impl iced::widget::button::StyleSheet for ActiveTab {
  type Style = Theme;

  fn active(&self, style: &Self::Style) -> widget::button::Appearance {
    style.hovered(&iced::theme::Button::Primary)
  }
}


pub struct NewTabButton;

impl widget::button::StyleSheet for NewTabButton {
  type Style = Theme;

  fn active(&self, style: &Self::Style) -> widget::button::Appearance {
      widget::button::Appearance {
        border: Border { radius: Radius::from(50.), ..Default::default() },
        ..style.active(&theme::Button::Primary)
      }
  }

  fn hovered(&self, style: &Self::Style) -> widget::button::Appearance {
    widget::button::Appearance {
        border: Border { radius: Radius::from(50.), ..Default::default() },
        ..style.hovered(&theme::Button::Primary)
      }    
  }

  fn disabled(&self, style: &Self::Style) -> widget::button::Appearance {
      widget::button::Appearance {
        border: Border { radius: Radius::from(50.), ..Default::default() }, 
        ..style.disabled(&theme::Button::Primary)
      }
  }

  fn pressed(&self, style: &Self::Style) -> widget::button::Appearance {
      widget::button::Appearance {
        border: Border { radius: Radius::from(50.), ..Default::default() }, 
        ..style.pressed(&theme::Button::Primary)
      }
  }

}

pub struct NumberColitionSelected;

impl widget::button::StyleSheet for NumberColitionSelected {
  type Style = Theme;

  fn active(&self, style: &Self::Style) -> widget::button::Appearance {
    style.hovered(&theme::Button::Destructive)
  }
}