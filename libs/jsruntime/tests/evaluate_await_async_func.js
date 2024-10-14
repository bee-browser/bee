print(1); ///#0=1
a();
print(4); ///#3=4

async function a() {
  print(2); ///#1=2
  await b();
  print(6); ///#5=6
}

async function b() {
  print(3); ///#2=3
  await 0;
  print(5); ///#4=5
}
