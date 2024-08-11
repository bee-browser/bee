let i = 0;
label: {
  switch (i) {
    case 0:
      i += 1;
      break label;
    default:
      i = 10;
      break;
  }
  i += 1;
}
print(i); ///=1
