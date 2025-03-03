#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/ExecutionEngine/Orc/ThreadSafeModule.h>
#pragma GCC diagnostic pop

#include "../bridge.hh"
#include "impl.hh"

#define IMPL(peer) reinterpret_cast<Module*>(peer)

void module_peer_delete(ModulePeer peer) {
  delete IMPL(peer);
}

void module_peer_print(ModulePeer peer, bool stderr) {
  IMPL(peer)->Print(stderr);
}
