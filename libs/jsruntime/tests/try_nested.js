let i = 0;
try {
  try {
    i += 1;
    throw i;
  } finally {
    i = 10;
  }
} catch (e) {
  i += e;
}
print(i);
