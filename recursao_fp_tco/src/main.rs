fn main() {
    print_all(0)
}

fn print_all(count: i64) {
    const ONE_HUNDRED_MILLION: i64 = 100_000_000;

    if count % 1_000_000 == 0 {
        println!("{count}");
    }
     
    if count <= ONE_HUNDRED_MILLION  {
        print_all(count + 1);
    }
}

