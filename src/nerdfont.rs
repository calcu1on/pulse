pub struct NerdFontIcon {
  pub icon_code: String,
} 

impl NerdFontIcon {
    /// Get the single font by name.
    pub fn get_icon(&self) -> Option<char> {
        u32::from_str_radix(&self.icon_code, 16).ok().and_then(char::from_u32)
    }
}
