use serde::{Serialize, Deserialize};
use tauri::Window;
use window_vibrancy::{
  apply_acrylic, apply_blur, apply_mica, clear_acrylic, clear_blur, clear_mica,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Effect {
  Blur,
  Acrylic,
  Mica,
}

pub fn set_effect(effect: Effect, window: &Window) {
  clear_blur(&window).unwrap();
  clear_acrylic(&window).unwrap();
  clear_mica(&window).unwrap();
  match effect {
    Effect::Blur => apply_blur(&window, Some((238, 238, 244, 100))).unwrap(),
    Effect::Acrylic => apply_acrylic(&window, Some((238, 238, 244, 100))).unwrap(),
    Effect::Mica => apply_mica(&window).unwrap(),
  };
}
