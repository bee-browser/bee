// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/jsruntime/src/bridge/runtime.hh.njk

typedef struct {
  bool (*to_boolean)(uintptr_t context, const Value* value);
  double (*to_numeric)(uintptr_t context, const Value* value);
  int32_t (*to_int32)(uintptr_t context, double value);
  uint32_t (*to_uint32)(uintptr_t context, double value);
  bool (*is_strictly_equal)(uintptr_t context, const Value* a, const Value* b);
} Runtime;
