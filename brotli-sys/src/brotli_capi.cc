#include <encode.h>

#include "brotli_capi.h"

extern "C" RustBrotliParams*
RustBrotliParamsCreate() {
  return reinterpret_cast<RustBrotliParams*>(new brotli::BrotliParams());
}

extern "C" void
RustBrotliParamsDestroy(RustBrotliParams *params) {
  delete reinterpret_cast<brotli::BrotliParams*>(params);
}

extern "C" void
RustBrotliParamsSetMode(RustBrotliParams *params,
                        RustBrotliMode mode) {
  brotli::BrotliParams::Mode BrotliMode;
  switch (mode) {
    case RUST_MODE_TEXT:
      BrotliMode = brotli::BrotliParams::MODE_TEXT;
      break;
    case RUST_MODE_FONT:
      BrotliMode = brotli::BrotliParams::MODE_FONT;
      break;
    case RUST_MODE_GENERIC:
    default:
      BrotliMode = brotli::BrotliParams::MODE_GENERIC;
      break;
  }
  reinterpret_cast<brotli::BrotliParams*>(params)->mode = BrotliMode;
}

extern "C" void
RustBrotliParamsSetQuality(RustBrotliParams *params, int quality) {
  reinterpret_cast<brotli::BrotliParams*>(params)->quality = quality;
}

extern "C" void
RustBrotliParamsSetLgwin(RustBrotliParams *params, int lgwin) {
  reinterpret_cast<brotli::BrotliParams*>(params)->lgwin = lgwin;
}

extern "C" void
RustBrotliParamsSetLgblock(RustBrotliParams *params, int lgblock) {
  reinterpret_cast<brotli::BrotliParams*>(params)->lgblock = lgblock;
}

extern "C" void
RustBrotliParamsSetEnableDictionary(RustBrotliParams *params, int enable) {
  reinterpret_cast<brotli::BrotliParams*>(params)->enable_dictionary = enable;
}

extern "C" void
RustBrotliParamsSetEnableTransforms(RustBrotliParams *params, int enable) {
  reinterpret_cast<brotli::BrotliParams*>(params)->enable_transforms = enable;
}

extern "C" void
RustBrotliParamsSetGreedyBlockSplit(RustBrotliParams *params, int split) {
  reinterpret_cast<brotli::BrotliParams*>(params)->greedy_block_split = split;
}

extern "C" void
RustBrotliParamsSetEnableContextModeling(RustBrotliParams *params, int enable) {
  reinterpret_cast<brotli::BrotliParams*>(params)->enable_context_modeling = enable;
}

extern "C" int
RustBrotliCompressBuffer(RustBrotliParams *params,
                         size_t input_size,
                         const uint8_t* input_buffer,
                         size_t* encoded_size,
                         uint8_t* encoded_buffer) {
  brotli::BrotliParams *Params = reinterpret_cast<brotli::BrotliParams*>(params);
  return brotli::BrotliCompressBuffer(*Params, input_size, input_buffer,
                                      encoded_size, encoded_buffer);
}

class MyBrotliOut : public brotli::BrotliOut {
 public:
  MyBrotliOut(void *data, int (*callback)(void*, const void*, size_t))
    : data(data), callback(callback)
  {}

  bool Write(const void* buf, size_t n) {
    return callback(data, buf, n);
  }

 private:
  void *data;
  int (*callback)(void*, const void*, size_t);
};

extern "C" int
RustBrotliCompressBufferVec(RustBrotliParams *params,
                            size_t input_size,
                            const uint8_t* input_buffer,
                            void *data,
                            int(*callback)(void*, const void*, size_t)) {
  brotli::BrotliMemIn Input(input_buffer, input_size);
  MyBrotliOut Output(data, callback);
  brotli::BrotliParams *Params = reinterpret_cast<brotli::BrotliParams*>(params);

  return brotli::BrotliCompress(*Params, &Input, &Output);
}
