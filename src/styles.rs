use iced::{border::Radius, widget::{self, button::StyleSheet}, Theme};


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
