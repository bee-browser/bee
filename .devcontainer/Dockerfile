FROM mcr.microsoft.com/vscode/devcontainers/rust

RUN apt-get update \
 && export DEBIAN_FRONTEND=noninteractive \
 && apt-get -y install --no-install-recommends curl make jq yq \
 && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
 && cargo binstall -y grcov \
 && curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C $CARGO_HOME/bin \
 # We have to do `chmod a+w` in order to avoid permission issues.
 && chmod -R a+w $RUSTUP_HOME $CARGO_HOME \
 && curl -fsSL https://deno.land/x/install/install.sh | DENO_INSTALL=/usr/local/deno sh \
 && apt-get autoremove -y \
 && apt-get clean -y \
 && rm -rf /var/lib/apt/lists/*

ENV DENO_DIR=/tmp/deno
ENV PATH=/usr/local/deno/bin:$PATH

# WORKAROUND
# ----------
# For the `lldb.launch.sourceMap` property defined in .vscode/settings.json, the
# following environment variables must be defined on a remote container before
# a debugger starts.
#
#   * RUSTC_COMMIT_HASH
#   * RUST_TOOLCHAIN_PATH
#
# The devcontainer.json has properties to define environment variables on the
# remote container.  However, none of them work properly with the CodeLLDB
# extension.  When trying to start a debug session, the following error message
# will be shown on the debug console:
#
#   Could not set source map: the replacement path doesn't exist: "<path>"
#
# even though "<path>" exists on the remote container.
#
# Directly setting the target.source-map by using
# settings.'lldb.launch.xxxCommands' also outputs the same error message.
#
# Exporting the environment variables by using the CMD instruction doesn't work.
# The environment variables are not defined on a debuggee process.  Because the
# debuggee process is NOT a child process of the init process which executes a
# script of the CMD instruction.
#
# The only way to solve this issue is providing the values for the environment
# variables from somewhere outside the system.
ENV RUSTC_COMMIT_HASH="9b00956e56009bab2aa15d7bff10916599e3d6d6"
ENV RUST_TOOLCHAIN_PATH="/usr/local/rustup/toolchains/1.78.0-x86_64-unknown-linux-gnu"
