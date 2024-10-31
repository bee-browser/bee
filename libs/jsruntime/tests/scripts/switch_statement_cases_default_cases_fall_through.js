let i = -1;
switch (i) {
  case 0:
    print(0);
    // fall through
  default:
    print(-1); ///=-1
    // fall through
  case 1:
    print(1); ///=1
    // fall through
}
print(100); ///=100
