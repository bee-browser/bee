let b = a();
print(b());
function a() {
  let x = 1;
  return () => x;
}
