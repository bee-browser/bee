let i = 0;
outer:
for (let j = 0; j < 2; j++) {
  i++;
  inner:
  for (;;) {
    continue outer;
  }
  j++;
}
print(i);
