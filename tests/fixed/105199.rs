pub trait Stream {
    type Item;
    type Error;
}

pub trait ParseError<I> {
    type Output;
}

impl ParseError<char> for u32 {
    type Output = ();
}

impl Stream for () {
    type Item = char;
    type Error = char;
}

pub struct Lex<'a, I>
    where I: Stream,
          I::Error: ParseError<char>,
          <<char as Stream>::Error as ParseError<I::Item>>::Output: 'a
{
    x: &'a <I::Error as ParseError<char>>::Output
}

pub struct Reserved<'a, I> where
    I: ParseError<char> + 'a,
    I: ParseError<char> + 'a,
    <<I as Stream>::Error as ParseError<char>>::Output: 'a

{
    x: Lex<'a, I>
}

fn main() {
    let r: Reserved<()> = Reserved {
        x: Reserved {
            x: Reserved {
             x: &()
        }
        }
    };

    let _v = r.x.x;
}
