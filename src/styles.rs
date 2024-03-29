use iced::{border::Radius, widget, Theme};


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

