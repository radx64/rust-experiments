mod pair;

use pair::Pair;

struct Foo{

}

fn main() {

    let p = Pair::new(10, 20);
    p.cmp_display();

    let c = Pair::new('c', 'd');
    c.cmp_display();

    let s = Pair::new("zzzz", "aaaa");
    s.cmp_display();

    let s: Pair<Foo> = Pair::new(Foo{}, Foo{});
    // s.cmp_display(); this will not compile as Foo does not implements PartialOrd

    println!("Hello, world!");
}
