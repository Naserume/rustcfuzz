// incremental
#![allow(dead_code)]

trait Query {
    type Value;
}

struct Slot<Q>
where
    Q: Query,
{
    state: std::cell::Cell<QueryState<Q>>,
}

enum QueryState<Q>
where
    Q: Query,
{
    InProgress(Q::Value),
    Memoized(Memo<Q>),
}

struct Memo<Q>
where
    Q: Query,
{
    value: Q::Value,
    inputs: Box<()>,
}

struct HirNodeQuery;

impl Query for HirNodeQuery {
    type Value = HirNode;
}

enum HirNode {
    A,
    #[cfg(a)]
    B(()),
}

fn conjure<T>() -> T {
    todo!()
}

fn main() {
    let _slot = conjure::<Slot<HirNodeQuery>>();
}
