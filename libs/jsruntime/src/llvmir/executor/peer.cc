#include "../bridge.hh"
#include "../module/impl.hh"
#include "impl.hh"

#define PEER(impl) reinterpret_cast<ExecutorPeer>(impl)
#define IMPL(peer) reinterpret_cast<Executor*>(peer)

ExecutorPeer executor_peer_new() {
  return PEER(llvm::cantFail(Executor::Create()));
}

void executor_peer_delete(ExecutorPeer peer) {
  delete IMPL(peer);
}

void executor_peer_register_runtime_functions(ExecutorPeer peer,
                                              const RuntimeFunctions* functions) {
  IMPL(peer)->RegisterRuntimeFunctions(functions);
}

void executor_peer_register_host_function(ExecutorPeer peer, uint32_t func_id, Lambda lambda) {
  IMPL(peer)->RegisterHostFunction(func_id, lambda);
}

void executor_peer_register_module(ExecutorPeer peer, ModulePeer mod) {
  IMPL(peer)->RegisterModule(reinterpret_cast<Module*>(mod));
}

const char* executor_peer_get_data_layout(const ExecutorPeer peer) {
  return IMPL(peer)->data_layout().getStringRepresentation().c_str();
}

const char* executor_peer_get_target_triple(const ExecutorPeer peer) {
  return IMPL(peer)->target_triple().getTriple().c_str();
}

Lambda executor_peer_get_native_function(ExecutorPeer peer, uint32_t func_id) {
  return IMPL(peer)->GetNativeFunction(func_id);
}
