let i = 0;
outer:
for (;;) {
  inner:
  for (;;) {
    break outer;
  }
  i = 1;
}
print(i);
