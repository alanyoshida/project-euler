#include <stdio.h>
#include <string.h>
#include <stdbool.h>

bool isPrime(int n) {
	for(int j = 2; j < n; j++){
		if(n%j == 0){
			return false;
		}
	}
	return true;
}

int largestPrimeFactor(int n) {
	int out = 0;
	for(int i = 1; i*i < n; i++){
		if(n%i == 0 && isPrime(i)){
				out = i;
    }
	}
	return out;
}

void testIsPrime() {
  printf("\n\n\e[1m## %s \e[0m\n", __func__);
  struct Test{
    bool returned;
    bool want;
  };
  const int SIZE = 4;
  struct Test unitTests[SIZE];

  // Validations
  unitTests[0].returned = isPrime(3);
  unitTests[0].want = true;

  unitTests[1].returned = isPrime(5);
  unitTests[1].want = true;

  unitTests[2].returned = isPrime(7);
  unitTests[2].want = true;

  unitTests[2].returned = isPrime(4);
  unitTests[2].want = false;

  for(int i=0;i<SIZE;i++){
    if( unitTests[i].returned == unitTests[i].want){
     printf("\n\e[1m\e[32mReturned: %d, Expected: %d - Test passed\e[0m",unitTests[i].returned, unitTests[i].want);
    }else {
     printf("\n\e[1m\e[31mReturned: %d, Expected: %d - Test failed\e[0m",unitTests[i].returned, unitTests[i].want);
    }
  }
}

void testLargestPrimeFactor() {
  printf("\n\n\e[1m## %s \e[0m\n", __func__);
  struct Test{
    int returned;
    int want;
  };
  const int SIZE = 1;
  struct Test unitTests[SIZE];

  // Validations
  unitTests[0].returned = largestPrimeFactor(13195);
  unitTests[0].want = 29;

  for(int i=0;i<SIZE;i++){
    if( unitTests[i].returned == unitTests[i].want){
     printf("\n\e[1m\e[32mReturned: %d, Expected: %d - Test passed",unitTests[i].returned, unitTests[i].want);
    }else {
     printf("\n\e[1m\e[31mReturned: %d, Expected: %d - Test failed",unitTests[i].returned, unitTests[i].want);
    }
  }
}

void main( int argc, char *argv[ ]){
  testIsPrime();
  testLargestPrimeFactor();
  printf("\n\nLargest prime factor of 600851475143 = %d", largestPrimeFactor(13195));
}
