// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/
use std::rc::Rc;
extern crate core;
use core::slice::Iter;
use core::ops::{Deref, Range};

pub struct RCSlice<T>{
    slice: Rc<[T]>,
    range: Range<u16>,
}

impl<T> RCSlice<T>{
    #[inline]
    pub fn new<U: Into<Rc<[T]>>>(that: U, range: Range<usize>) -> RCSlice<T>{   
        let range = (range.start as u16)..(range.end as u16);
        let slice = that.into();
        RCSlice{
            slice,
            range,
        }
    }    
    #[inline]
    pub fn as_slice(&self) -> &[T]{                                             
        let range = (self.range.start as usize)..(self.range.end as usize);     
        &(*self.slice)[range]
    }
    pub fn iter(&self) -> Iter<'_, T>{                                     
        self.as_slice().iter()                                                  
    } 
}

impl<T> Deref for RCSlice<T>{
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T]{
        self.as_slice()
    }
}

#[inline]
const fn lex_break(b: &u8) -> bool{
    *b == b' ' || *b == b'\t'
}

#[inline]
fn not_empty(that: (usize, &[u8])) -> bool{
    !that.1.is_empty()
}

fn parse<I>(
    iter: &mut I,
) -> Result<(), ()>
where
    I: Iterator<Item = RCSlice<u8>>
{
    for i in iter{
        let mut chunks = i.split(lex_break).enumerate().filter(not_empty);
        let mut i = 0;
        for (i, b) in chunks{

        }
    }
    Ok(())
}
