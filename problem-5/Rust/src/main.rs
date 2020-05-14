fn main() {
  println!("smallest_multiple = {}", smallest_multiple(20));
}

fn smallest_multiple(upper_bound: i32) -> i32 {
let mut x: i32 = 0;
loop {
  x += 1;
  if is_divisible_by_range(x, upper_bound) {
    return x
  }
}
}

fn is_divisible_by_range(testd: i32, max: i32) -> bool {
for y in 1..max {
  if testd%y != 0 {
    return false;
  }
}
return true;
}

#[test]
fn test_is_divisible_by_range(){
assert_eq!(is_divisible_by_range(2520, 10), true);
assert_eq!(is_divisible_by_range(2, 10), false);
}

#[test]
fn test_smallest_multiple(){
assert_eq!(smallest_multiple(10), 2520);
}

