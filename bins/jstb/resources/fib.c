#include <stdio.h>
#include <stdlib.h>

int fib(int n) {
  if (n < 2) {
    return n;
  }
  return fib(n - 1) + fib(n - 2);
}

int main(int argc, char** argv) {
  (void)argc;
  int n = fib(atoi(argv[1]));
  printf("%d\n", n);
  return 0;
}
