# See https://apt.llvm.org/#llvmsh.
sudo apt-get install -y lsb-release wget software-properties-common gnupg
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
llvm-config-18 --version

# //libs/jsruntime/build.sh uses //vendor/bin/llvm-config.
mkdir -p vendor/bin
ln -sf $(which llvm-config-18) vendor/bin/llvm-config
