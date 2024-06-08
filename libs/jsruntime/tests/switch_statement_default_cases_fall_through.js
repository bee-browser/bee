let i = -1;
switch (i) {
  default:
    i = 1;
    // fall through
  case 0:
    i = 2;
    // fall through
  case 1:
    i = 3;
    // fall through
}
print(i);
