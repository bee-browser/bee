let i = 0, j = 0;
outer:
nested:
while (j < 2) {
  i++;
  j++;
  inner:
  for (;;) {
    if (i == 1) {
      continue nested;
    } else {
      continue outer;
    }
  }
  j++;
}
print(i);
