let i = 0;
switch (i) {
  case 0:
    i = 1;
    // fall through
  case 1:
    i = 2;
    // fall through
  default:
    i = 3;
    // fall through
}
print(i);
