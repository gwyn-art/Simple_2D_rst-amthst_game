
#[derive (PartialEq, Eq)]
pub enum BuildMode {
  Realise,
  Debug
}

pub struct BuildSettings {
  build_mode: BuildMode
}

impl BuildSettings {
  pub fn build_mode(&self) -> &BuildMode {
    &self.build_mode
  }
}


// As it is hard to declare complex constant I used function here
pub fn get_build_settings () -> BuildSettings {
  BuildSettings {
    build_mode: BuildMode::Realise
  }
}