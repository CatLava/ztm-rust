use serde::Serialize;
use derive_more::Constructor;

pub trait PageContext {
    // this is the content of the page
    fn title(&self) -> &str;
    // path to usage template
    fn template_path(&self) -> &str;
    // keeping websites the same
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize)]
pub struct Home {

}

impl Default for Home {
    fn default() -> Self {
        Self{}
    }
}

impl PageContext for Home {
    fn template_path(&self) -> &str {
        "home"
    }

    fn title(&self) -> &str {
        "Stash your clipboard"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: crate::Clip,
}

impl PageContext for ViewClip {
    fn template_path(&self) -> &str {
        "clip"
    }

    fn title(&self) -> &str {
        "View your clip"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]

pub struct PasswordRequired {
    shortcode: crate::ShortCode,
}

impl PageContext for PasswordRequired {
    fn template_path(&self) -> &str {
        "clip_need_password"
    }

    fn title(&self) -> &str {
        "password required"
    }

    fn parent(&self) -> &str {
        "base"
    }
}