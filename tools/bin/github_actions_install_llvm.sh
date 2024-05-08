# See https://apt.llvm.org/#llvmsh.
sudo apt-get install -y lsb-release wget software-properties-common gnupg
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18

# Install packages needed for //libs/jsruntime/build.sh to work.
# Static libraries are needed.
sudo apt-get install -y libtinfo-dev libxml2-dev zlib1g-dev libzstd-dev

# //libs/jsruntime/build.sh uses //vendor/bin/llvm-config.
mkdir -p vendor/bin
ln -sf $(which llvm-config-18) vendor/bin/llvm-config

# tests
COMPONENTS='core orcjit x86'
LINK_TYPE='--link-static'
vendor/bin/llvm-config --version
vendor/bin/llvm-config --assertion-mode
vendor/bin/llvm-config --build-mode
vendor/bin/llvm-config --host-target
vendor/bin/llvm-config --targets-built
vendor/bin/llvm-config $LINK_TYPE --cxxflags
vendor/bin/llvm-config $LINK_TYPE --libdir
vendor/bin/llvm-config $LINK_TYPE --libs $COMPONENTS
vendor/bin/llvm-config $LINK_TYPE --system-libs $COMPONENTS
