#[derive(Debug, PartialOrd, PartialEq, Clone)]

pub struct Field(pub u32, pub u32);

impl Field {
    pub fn none() -> Self {
        Self(0, 0)
    }
    pub fn merge(&mut self, field: Self) -> () {
        self.0 |= field.0;
        self.1 |= field.1;
    }
    pub fn add(&mut self, flag: &Flag) -> () {
        match flag {
            Flag::Flag(first) => self.0 |= first,
            Flag::Second(second) => self.1 |= second,
        };
    }
    pub fn present(&self, flag: &Flag) -> bool {
        match flag {
            Flag::Flag(first) => (first & self.0) == *first,
            Flag::Second(second) => (second & self.1) == *second,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Flag {
    Flag(u32),
    Second(u32),
}

impl Flag {
    pub fn value(&self) -> u32 {
        match self {
            Self::Flag(first) => (*first as f32).log2().round() as u32,
            Self::Second(second) => (*second as f32).log2().round() as u32 + 32,
        }
    }
    pub fn from(i: u32) -> Self {
        if i > 31 {
            Self::Second(1 << (i - 32))
        } else {
            Self::Flag(1 << i)
        }
    }
    // Used for Style invalidation
    pub const fn transparency() -> Flag {
        Flag::Flag(1)
    }
    pub const fn padding() -> Flag {
        Flag::Flag(2)
    }
    pub const fn spacing() -> Flag {
        Flag::Flag(3)
    }
    pub const fn font_size() -> Flag {
        Flag::Flag(4)
    }
    pub const fn font_family() -> Flag {
        Flag::Flag(5)
    }
    pub const fn width() -> Flag {
        Flag::Flag(6)
    }
    pub const fn height() -> Flag {
        Flag::Flag(7)
    }
    pub const fn bg_color() -> Flag {
        Flag::Flag(8)
    }
    pub const fn bg_image() -> Flag {
        Flag::Flag(9)
    }
    pub const fn bg_gradient() -> Flag {
        Flag::Flag(10)
    }
    pub const fn border_style() -> Flag {
        Flag::Flag(11)
    }
    pub const fn font_alignment() -> Flag {
        Flag::Flag(12)
    }
    pub const fn font_weight() -> Flag {
        Flag::Flag(13)
    }
    pub const fn font_color() -> Flag {
        Flag::Flag(14)
    }
    pub const fn font_spacing() -> Flag {
        Flag::Flag(15)
    }
    pub const fn letter_spacing() -> Flag {
        Flag::Flag(16)
    }
    pub const fn border_rount() -> Flag {
        Flag::Flag(17)
    }
    pub const fn text_shadows() -> Flag {
        Flag::Flag(18)
    }
    pub const fn shadows() -> Flag {
        Flag::Flag(19)
    }
    pub const fn overflow() -> Flag {
        Flag::Flag(20)
    }
    pub const fn cursor() -> Flag {
        Flag::Flag(21)
    }
    pub const fn scale() -> Flag {
        Flag::Flag(23)
    }
    pub const fn rotate() -> Flag {
        Flag::Flag(24)
    }
    pub const fn move_x() -> Flag {
        Flag::Flag(25)
    }
    pub const fn move_y() -> Flag {
        Flag::Flag(26)
    }
    pub const fn border_width() -> Flag {
        Flag::Flag(27)
    }
    pub const fn border_color() -> Flag {
        Flag::Flag(28)
    }
    pub const fn align_y() -> Flag {
        Flag::Flag(29)
    }
    pub const fn align_x() -> Flag {
        Flag::Flag(30)
    }
    pub const fn focus() -> Flag {
        Flag::Flag(31)
    }
    pub const fn active() -> Flag {
        Flag::Flag(32)
    }
    pub const fn hover() -> Flag {
        Flag::Flag(33)
    }
    pub const fn grid_template() -> Flag {
        Flag::Flag(34)
    }
    pub const fn grid_position() -> Flag {
        Flag::Flag(35)
    }
    // Notes
    pub const fn height_content() -> Flag {
        Flag::Flag(36)
    }
    pub const fn height_fill() -> Flag {
        Flag::Flag(37)
    }
    pub const fn width_content() -> Flag {
        Flag::Flag(38)
    }
    pub const fn width_fill() -> Flag {
        Flag::Flag(39)
    }
    pub const fn align_right() -> Flag {
        Flag::Flag(40)
    }
    pub const fn align_bottom() -> Flag {
        Flag::Flag(41)
    }
    pub const fn center_x() -> Flag {
        Flag::Flag(42)
    }
    pub const fn center_y() -> Flag {
        Flag::Flag(43)
    }
    pub const fn width_between() -> Flag {
        Flag::Flag(44)
    }
    pub const fn height_between() -> Flag {
        Flag::Flag(45)
    }
    pub const fn behind() -> Flag {
        Flag::Flag(46)
    }
    pub const fn height_text_area_content() -> Flag {
        Flag::Flag(47)
    }
    pub const fn font_variant() -> Flag {
        Flag::Flag(48)
    }
}
