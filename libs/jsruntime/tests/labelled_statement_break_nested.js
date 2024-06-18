let i = 0;
outer:
nested:
{
  inner:
  for (;;) {
    break nested;
  }
  i = 1;
}
print(i);
