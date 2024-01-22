fn main() {
    let tup = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    let typed_tup: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = typed_tup.0;
    let _six_point_four = typed_tup.1;
    let _one = typed_tup.2;

    let _empty_tup: () = ();//a "unit" which is an empty tuple
}
