#include <stdio.h>
#include <string.h>

int fibo(int n){
  int a = 1, b = 1, c = 1, sum = 0;

  while( c < n ) {
    b = c;
    c = a + b;
    a = b;
    if (c%2 == 0) {
      sum += c;
    }
  }
  return sum;
}

void test() {
  struct Test{
    int returned;
    int want;
  };
  const int SIZE = 1;
  struct Test unitTests[SIZE];

  // Validations
  unitTests[0].returned = fibo(10);
  unitTests[0].want = 10;

  for(int i=0;i<SIZE;i++){
    if( unitTests[i].returned == unitTests[i].want){
     printf("\n\e[1m\e[32mReturned: %d, Expected: %d - Test passed",unitTests[i].returned, unitTests[i].want);
    }else {
     printf("\n\e[1m\e[31mReturned: %d, Expected: %d - Test failed",unitTests[i].returned, unitTests[i].want);
    }
  }
}

void main( int argc, char *argv[ ]){
  test();
  printf("\n\nsum of even numbers of less than 4 million = %d", fibo(4000000));
}
