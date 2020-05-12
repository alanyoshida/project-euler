fn main() {
  println!("sum of even numbers of less than 4 million = {}", fibo(4000000));
}

fn fibo(n: i32) -> i32 {
  let mut a: i32 = 1;
  let mut b: i32 = 1;
  let mut c: i32 = 1;
  let mut sum: i32 = 0;
  while c < n {
    b = c;
    c = a + b;
    a = b;
    if c%2 == 0 {
      sum += c;
    }
    println!("a = {}, b = {}, c = {}", a, b, c);
  }
  return sum;
}

#[test]
fn from_first_10(){
  assert_eq!(fibo(10), 10);
}

