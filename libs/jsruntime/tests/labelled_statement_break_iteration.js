outer:
for (;;) {
  inner:
  for (;;) {
    break outer;
  }
}
print(0);
