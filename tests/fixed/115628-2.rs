extern "C" fn foo<T: ?Sized + 'static>() -> Option<&'static T> { None }
