trait Trait2 : Trait {
	reuse <() as Trait>::async {
		(async || {}).await;
	};
}
