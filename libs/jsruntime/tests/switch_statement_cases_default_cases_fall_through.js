let i = -1;
switch (i) {
  case 0:
    i = 1;
    // fall through
  default:
    i = 2;
    // fall through
  case 1:
    i = 3;
    // fall through
}
print(i);
