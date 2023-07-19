use crate::vec3;
use std::fmt;

// XXX: in the original c++, this is writing to the output descriptor, and in my
// case I'm copying over the trait implementation of fmt for Vec3, I don't know
// if I need to return the result.

type Color = Vec3;

// At this point, I'm assuming that this associated type is only for
// Color, I'm not sure if the vec3 implementations will carry over.

impl fmt::Display for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the translated [0,255] value of each color component
        formatter.write_fmt(format_args!(
            "{} {} {}",
            (self.x * 255.999) as i32,
            (self.y * 255.999) as i32,
            (self.z * 255.999) as i32,
        ))
    }
}
