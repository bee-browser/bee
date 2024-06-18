let i = 0, j = 0;
outer:
do {
  i++;
  j++;
  inner:
  for (;;) {
    continue outer;
  }
  j++;
} while (i < 2);
print(i);
