fn main() {
    println!("result={}", multiples_of_three_and_five_until(1000));
}

fn multiples_of_three_and_five_until(n:i32) -> i32 {
  let mut sum: i32 = 0;
  for i in 0..n {
    if i%3 == 0 || i%5 ==0 {
      sum += i;
    }
  }
  return sum;
}

#[test]
fn passing_10(){
  assert_eq!(multiples_of_three_and_five_until(10), 23);
}

#[test]
fn passing_1000(){
  assert_eq!(multiples_of_three_and_five_until(1000), 233168);
}
