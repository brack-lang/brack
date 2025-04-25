use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Html {
    pub tag: HtmlTag,
    pub attributes: Vec<HtmlAttribute>,
    pub children: Vec<Html>,
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ", self.tag)?;
        for attr in &self.attributes {
            write!(f, "{} ", attr)?;
        }
        write!(f, ">")?;
        for child in &self.children {
            write!(f, "{}", child)?;
        }
        write!(f, "</{}>", self.tag)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HtmlTag {
    Fragment(String),
    Div,
    Span,
    P,
    B,
}

impl fmt::Display for HtmlTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlTag::Fragment(_) => write!(f, ""),
            HtmlTag::Div => write!(f, "div"),
            HtmlTag::Span => write!(f, "span"),
            HtmlTag::P => write!(f, "p"),
            HtmlTag::B => write!(f, "b"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HtmlAttribute {
    Class,
    Id,
    Style(HtmlStyle),
}

impl fmt::Display for HtmlAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlAttribute::Class => write!(f, "class"),
            HtmlAttribute::Id => write!(f, "id"),
            HtmlAttribute::Style(style) => {
                write!(f, "style")?;
                match style {
                    HtmlStyle::FontStyle(font_style) => {
                        write!(f, "font-style: {}", font_style)?;
                    }
                    HtmlStyle::FontWeight(font_weight) => {
                        write!(f, "font-weight: {}", font_weight)?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HtmlStyle {
    #[deprecated(note = "This property is deprecated and non-standard.")]
    MozFloatEdge(HtmlStyleMozFloatEdge),
    #[deprecated(note = "This property is deprecated and non-standard.")]
    MozForceBrokenImageIcon(HtmlStyleMozForceBrokenImageIcon),
    MozImageRegion(HtmlStyleMozImageRegion),
    FontStyle(HtmlStyleFontStyle),
    FontWeight(HtmlStyleFontWeight),
}

/// https://developer.mozilla.org/ja/docs/Web/CSS/-moz-float-edge
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HtmlStyleMozFloatEdge {
    BorderBox,
    ContentBox,
    MarginBox,
    PaddingBox,
}

impl fmt::Display for HtmlStyleMozFloatEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlStyleMozFloatEdge::BorderBox => write!(f, "border-box"),
            HtmlStyleMozFloatEdge::ContentBox => write!(f, "content-box"),
            HtmlStyleMozFloatEdge::MarginBox => write!(f, "margin-box"),
            HtmlStyleMozFloatEdge::PaddingBox => write!(f, "padding-box"),
        }
    }
}

/// Initial value for `-moz-float-edge`
impl Default for HtmlStyleMozFloatEdge {
    fn default() -> Self {
        HtmlStyleMozFloatEdge::ContentBox
    }
}

/// https://developer.mozilla.org/ja/docs/Web/CSS/-moz-force-broken-image-icon
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HtmlStyleMozForceBrokenImageIcon {
    Value(u32)
}

impl fmt::Display for HtmlStyleMozForceBrokenImageIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlStyleMozForceBrokenImageIcon::Value(value) => write!(f, "{}", value),
        }
    }
}

/// Initial value for `-moz-force-broken-image-icon`
impl Default for HtmlStyleMozForceBrokenImageIcon {
    fn default() -> Self {
        HtmlStyleMozForceBrokenImageIcon::Value(0)
    }
}

/// https://developer.mozilla.org/ja/docs/Web/CSS/-moz-image-region
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HtmlStyleMozImageRegion {
    Auto,
    
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HtmlStyleFontStyle {
    Normal,
    Italic,
    Oblique(Option<HtmlStyleUnit>),
    Inherit,
    Initial,
    Revert,
    RevertLayer,
    Unset,
}

impl fmt::Display for HtmlStyleFontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlStyleFontStyle::Normal => write!(f, "normal"),
            HtmlStyleFontStyle::Italic => write!(f, "italic"),
            HtmlStyleFontStyle::Oblique(angle) => {
                if let Some(angle) = angle {
                    write!(f, "oblique {}", angle)
                } else {
                    write!(f, "oblique")
                }
            }
            HtmlStyleFontStyle::Inherit => write!(f, "inherit"),
            HtmlStyleFontStyle::Initial => write!(f, "initial"),
            HtmlStyleFontStyle::Revert => write!(f, "revert"),
            HtmlStyleFontStyle::RevertLayer => write!(f, "revert-layer"),
            HtmlStyleFontStyle::Unset => write!(f, "unset"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HtmlStyleUnit {
    Px(u32),
    Em(f32),
    Rem(f32),
    Percent(f32),
}

impl fmt::Display for HtmlStyleUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlStyleUnit::Px(value) => write!(f, "{}px", value),
            HtmlStyleUnit::Em(value) => write!(f, "{}em", value),
            HtmlStyleUnit::Rem(value) => write!(f, "{}rem", value),
            HtmlStyleUnit::Percent(value) => write!(f, "{}%", value),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum HtmlStyleFontWeight {
    Normal,
    Bold,
    Value(u32), // 0-1000
    Lighter,
    Bolder,
    Inherit,
    Initial,
    Revert,
    RevertLayer,
    Unset,
}

impl fmt::Display for HtmlStyleFontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlStyleFontWeight::Normal => write!(f, "normal"),
            HtmlStyleFontWeight::Bold => write!(f, "bold"),
            HtmlStyleFontWeight::Value(value) => write!(f, "{}", value),
            HtmlStyleFontWeight::Lighter => write!(f, "lighter"),
            HtmlStyleFontWeight::Bolder => write!(f, "bolder"),
            HtmlStyleFontWeight::Inherit => write!(f, "inherit"),
            HtmlStyleFontWeight::Initial => write!(f, "initial"),
            HtmlStyleFontWeight::Revert => write!(f, "revert"),
            HtmlStyleFontWeight::RevertLayer => write!(f, "revert-layer"),
            HtmlStyleFontWeight::Unset => write!(f, "unset"),
        }
    }
}
