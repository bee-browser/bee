print(0); ///#0=0
a();
print(2); ///#2=2

async function a() {
  print(1); ///#1=1
  throw 100;
}
