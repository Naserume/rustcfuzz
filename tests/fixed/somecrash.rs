#![feature(associated_const_equality)]
#![no_std]

use core::PhantomData::PhantomData;

pub struct NoPin;

pub trait SetAlternate<const A: u8> {}

impl Pins<TA> for NoPin {
    const A: u8;
}

pub trait PinA<PER> {
    const A: u8;
}

impl<PER> PinA<PER> for NoPin {
    const A: u8 = 0;
}

pub trait Pins<USART> {}

impl<USART, T, const TA: u8> Pins<USART> for T where
    T: PinA<USART, A = { TA }> + SetAlternate<TA>
{
}

struct Serial<USART>(PhantomData<USART>);

impl<USART> Pins<USART> where NoPin: Pins<USART> {}
