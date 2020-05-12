#include <stdio.h>
#include <string.h>

int multiplesOf(int n){
  int sum = 0;
  for (int i=1; i<n; i++){
    if(i%3 == 0 || i%5 == 0){
      sum += i;
      //printf("\ni=%d, sum=%d", i, sum);
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
  unitTests[0].returned = multiplesOf(10);
  unitTests[0].want = 23;

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
  printf("\nresult= %d", multiplesOf(1000));
}

