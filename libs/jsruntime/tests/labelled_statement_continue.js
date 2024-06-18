let i = 0;
outer:
while (i < 2) {
  i++;
  inner:
  for (;;) {
    continue outer;
  }
}
print(i);
