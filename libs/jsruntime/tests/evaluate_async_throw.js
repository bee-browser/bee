print(1); ///#0=1
a();
print(3); ///#2=3

async function a() {
  print(2); ///#1=2
  throw 100;
}
