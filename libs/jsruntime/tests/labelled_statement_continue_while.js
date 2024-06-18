let i = 0, j = 0;
outer:
while (j < 2) {
  i++;
  j++;
  inner:
  for (;;) {
    continue outer;
  }
  j++;
}
print(i);
