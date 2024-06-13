let i = 0;
try {
  throw 1;
} catch {
  i = 2;
}
print(i);
