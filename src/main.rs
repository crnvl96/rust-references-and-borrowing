struct Vector {
    x: i32,
}

fn main() {
    let mut v1: Vector = Vector { x: 1 };

    println!("Antes: v1.x = {}", v1.x);

    inc_x(&mut v1);

    println!("Depois: v1.x = {}", v1.x);
}

fn inc_x(v: &mut Vector) {
    v.x += 1;
}
