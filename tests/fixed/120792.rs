type WithLifetime<T> = impl Equals<>;
fn _defining_use<T>() -> WithLifetime<T> {}

trait Convert<'a> {
    
    fn convert<'b, T: ?Sized>(, x: &'a T) -> &'b T;
}



fn extend_lifetime<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    WithLifetime::<&'a ()>::convert_helper::<(), T>(&(), x)
}

trait Equals {
    
    fn convert_helper<'a, 'b, W: , T: ?Sized>(
        ,
        ,
    ) -> &'b T;
}

impl<S> Equals for S {
    
    fn convert_helper<'foo, 'b, W: Convert<'a, >, T: ?Sized>(
        proof: &'b Self,
        x: &'a T,
    ) -> &'b T {
        W::convert(proof, x)
    }
}
