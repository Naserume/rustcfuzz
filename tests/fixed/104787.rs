use core::array;

use futures_concurrency::prelude::*;
use futures_lite::{future::block_on, stream, StreamExt};

fn main() {
    block_on(async {
        // works, the array length is known
        // let s = from_fn::<_, 5, _>(stream::once)
        let s = array::from_fn(stream::once)
            .merge()
            .fold(0, |acc, n| acc + n)
            .await;

        assert_eq!(10, s);
    });
}
