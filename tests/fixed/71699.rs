fn foo([(), .., ()]: [(); 1 << 40]) {}

fn main() {
    foo([(); 1 << 40]);
}
