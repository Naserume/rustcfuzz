fn main() {
    let desc = TextureDescriptor(TextureFormat::A, 1, 1);
    device_create_texture(desc);
}

#[inline(never)]
#[no_mangle]
pub fn device_create_texture(desc: TextureDescriptor) {
    let format = desc.0;
    assert_eq!(format.required_features(desc.1), Feature::A);
}

#[derive(PartialEq, Eq, Debug)]
enum Feature {
    A,
    B,
}

#[repr(i32)]
pub enum TextureFormat {
    A,
    B,
    C,
    D,
}

impl TextureFormat {
    #[inline(never)]
    fn required_features(&self, v: i8) -> Feature {
        match self {
            Self::A => Feature::A,
            Self::B => Feature::B,
            Self::C => Feature::B,
            Self::D => match v {
                0 => Feature::A,
                _ => Feature::B,
            },
        }
    }
}

pub struct TextureDescriptor(pub TextureFormat, i8, i16);
