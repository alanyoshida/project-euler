fn main() {
    println!("largest_palindrome_product = {}", largest_palindrome_product(999, 999));
}

fn is_palindrome(number: i32) -> bool {
  let number_as_string = number.to_string();
  let reversed: String = number_as_string.chars().rev().collect();
  return number_as_string == reversed;
}

fn largest_palindrome_product(a: i32, b: i32) -> i32{
  let mut out: i32 = 0;
	for i in 0..=a {
		for j in 0..=b {
			let product = i * j;
			if is_palindrome(product) {
				if product > out {
					out = product;
				}
			}
		}
	}
	return out;
}


#[test]
fn must_return_true_9009_is_palindrome(){
  assert_eq!(is_palindrome(9009), true);
}

#[test]
fn must_return_false_9909_is_palindrome(){
  assert_eq!(is_palindrome(9909), false);
}

#[test]
fn must_return_9009(){
  assert_eq!(largest_palindrome_product(91, 99), 9009);
}

#[test]
fn must_return_product_of_999(){
  assert_eq!(largest_palindrome_product(999, 999), 906609);
}
