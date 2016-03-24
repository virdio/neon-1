#ifndef NEON_CLASS_METADATA_H_
#define NEON_CLASS_METADATA_H_

#include <stdint.h>
#include "v8.h"
#include "neon.h"


// Currently, Node only ever has one isolate so we could get away with storing
// Neon metadata in a global variable. But when workers land in Node, they will
// each have a separate isolate.
//
// See: https://github.com/nodejs/node/pull/2133
//
// So instead we have to store per-isolate metadata in one of the isolate's
// extensible data slots.
//
// Slots 0 and 1 appear to be reserved by Chrome, and slot 3 is reserved by Node.
// That apparently leaves only slot 2 available for storing Neon metadata.
//
// See: https://github.com/nodejs/node/blob/master/src/env.h#L33-L39
//
// If this causes clashes with some other consumer of V8 in the future, we try
// proposing to Node to make node::Environment extensible instead.

#define NEON_ISOLATE_SLOT 2


namespace neon {

class ClassMapHolder {
public:
  ClassMapHolder(void *map, NeonSys_DropCallback drop_map)
    : map_(map), drop_map_(drop_map)
  {
  }

  ~ClassMapHolder() {
    drop_map_(map_);
    map_ = nullptr;
  }

  void *GetMap() {
    return map_;
  }

private:
  void *map_;
  NeonSys_DropCallback drop_map_;
};


class ClassMetadata {
public:

  ClassMetadata(NeonSys_ConstructCallback construct_callback, void *construct_kernel, v8::FunctionCallback call_callback, void *call_kernel) {
    construct_callback_ = construct_callback;
    construct_kernel_ = construct_kernel;
    call_callback_ = call_callback;
    call_kernel_ = call_kernel;
  }

  void SetTemplate(v8::Isolate *isolate, v8::Local<v8::FunctionTemplate> t) {
    template_.Reset(isolate, t);
    template_.SetWeak(this, FinalizeTemplate, v8::WeakCallbackType::kParameter);
  }

  v8::Local<v8::FunctionTemplate> GetTemplate(v8::Isolate *isolate) {
    return v8::Local<v8::FunctionTemplate>::New(isolate, template_);
  }

  virtual bool construct(const v8::FunctionCallbackInfo<v8::Value>& info) = 0;

  // FIXME(PR): save a flag on `this` if it fails?
  void call(const v8::FunctionCallbackInfo<v8::Value>& info) {
    call_callback_(info);
  }

  void *GetCallKernel() {
    return call_kernel_;
  }

  void *GetConstructKernel() {
    return construct_kernel_;
  }

protected:

  virtual ~ClassMetadata() {
    template_.Reset();
  }

  NeonSys_ConstructCallback construct_callback_;
  void *construct_kernel_;
  v8::FunctionCallback call_callback_;
  void *call_kernel_;

private:

  static void FinalizeTemplate(const v8::WeakCallbackInfo<ClassMetadata>& data) {
    ClassMetadata *metadata = data.GetParameter();
    delete metadata;
  }

  v8::Global<v8::FunctionTemplate> template_;
  
};


class BaseClassInstanceMetadata {
public:

  BaseClassInstanceMetadata(v8::Isolate *isolate, v8::Local<v8::Object> instance, void *internals, NeonSys_DropCallback drop) {
    instance_.Reset(isolate, instance);
    instance_.SetWeak(this, FinalizeInstance, v8::WeakCallbackType::kParameter);
    internals_ = internals;
    drop_ = drop;
  }

  void *GetInternals() {
    return internals_;
  }

protected:
  ~BaseClassInstanceMetadata() {
    instance_.Reset();
    drop_(internals_);
    internals_ = nullptr;
  }

private:

  static void FinalizeInstance(const v8::WeakCallbackInfo<BaseClassInstanceMetadata>& data) {
    BaseClassInstanceMetadata *metadata = data.GetParameter();
    delete metadata;
  }

  void *internals_;
  v8::Global<v8::Object> instance_;
  NeonSys_DropCallback drop_;
};


class BaseClassMetadata: public ClassMetadata {
public:

  BaseClassMetadata(NeonSys_ConstructCallback construct_callback,
                    void *construct_kernel,
                    v8::FunctionCallback call_callback,
                    void *call_kernel,
                    NeonSys_AllocateCallback allocate_callback,
                    void *allocate_kernel,
                    NeonSys_DropCallback drop_instance)
    : ClassMetadata(construct_callback, construct_kernel, call_callback, call_kernel)
  {
    allocate_callback_ = allocate_callback;
    allocate_kernel_ = allocate_kernel;
    drop_instance_ = drop_instance;
  }

  void *GetAllocateKernel() {
    return allocate_kernel_;
  }

  // FIXME(PR): instead of returning bool, save a flag on `this`
  virtual bool construct(const v8::FunctionCallbackInfo<v8::Value>& info) {
    void *internals = allocate_callback_(&info);
    if (!internals) {
      return false;
    }
    v8::Local<v8::Object> self = info.This();
    BaseClassInstanceMetadata *instance = new BaseClassInstanceMetadata(info.GetIsolate(), self, internals, drop_instance_);
    self->SetAlignedPointerInInternalField(0, instance);
    return !construct_kernel_ || construct_callback_(&info);
  }

private:

  NeonSys_AllocateCallback allocate_callback_;
  void *allocate_kernel_;
  NeonSys_DropCallback drop_instance_;
  v8::Global<v8::Object> instance_;
};

// TODO: subclasses: class DerivedClassMetadata: public ClassMetadata { ... };

}; // end namespace neon

#endif
