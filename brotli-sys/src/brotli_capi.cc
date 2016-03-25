#include <encode.h>

#include "brotli_capi.h"

using namespace brotli;

extern "C" RustBrotliParams*
RustBrotliParamsCreate() {
  return reinterpret_cast<RustBrotliParams*>(new BrotliParams());
}

extern "C" void
RustBrotliParamsDestroy(RustBrotliParams *params) {
  delete reinterpret_cast<BrotliParams*>(params);
}

extern "C" void
RustBrotliParamsSetMode(RustBrotliParams *params,
                        RustBrotliMode mode) {
  BrotliParams::Mode BrotliMode;
  switch (mode) {
    case RUST_MODE_TEXT:
      BrotliMode = BrotliParams::MODE_TEXT;
      break;
    case RUST_MODE_FONT:
      BrotliMode = BrotliParams::MODE_FONT;
      break;
    case RUST_MODE_GENERIC:
    default:
      BrotliMode = BrotliParams::MODE_GENERIC;
      break;
  }
  reinterpret_cast<BrotliParams*>(params)->mode = BrotliMode;
}

extern "C" void
RustBrotliParamsSetQuality(RustBrotliParams *params, int quality) {
  reinterpret_cast<BrotliParams*>(params)->quality = quality;
}

extern "C" void
RustBrotliParamsSetLgwin(RustBrotliParams *params, int lgwin) {
  reinterpret_cast<BrotliParams*>(params)->lgwin = lgwin;
}

extern "C" void
RustBrotliParamsSetLgblock(RustBrotliParams *params, int lgblock) {
  reinterpret_cast<BrotliParams*>(params)->lgblock = lgblock;
}

extern "C" void
RustBrotliParamsSetEnableDictionary(RustBrotliParams *params, int enable) {
  reinterpret_cast<BrotliParams*>(params)->enable_dictionary = enable;
}

extern "C" void
RustBrotliParamsSetEnableTransforms(RustBrotliParams *params, int enable) {
  reinterpret_cast<BrotliParams*>(params)->enable_transforms = enable;
}

extern "C" void
RustBrotliParamsSetGreedyBlockSplit(RustBrotliParams *params, int split) {
  reinterpret_cast<BrotliParams*>(params)->greedy_block_split = split;
}

extern "C" void
RustBrotliParamsSetEnableContextModeling(RustBrotliParams *params, int enable) {
  reinterpret_cast<BrotliParams*>(params)->enable_context_modeling = enable;
}

extern "C" int
RustBrotliCompressBuffer(const RustBrotliParams *params,
                         size_t input_size,
                         const uint8_t* input_buffer,
                         size_t* encoded_size,
                         uint8_t* encoded_buffer) {
  const BrotliParams *Params = reinterpret_cast<const BrotliParams*>(params);
  return BrotliCompressBuffer(*Params, input_size, input_buffer,
                                      encoded_size, encoded_buffer);
}

class MyBrotliOut : public BrotliOut {
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
RustBrotliCompressBufferVec(const RustBrotliParams *params,
                            size_t input_size,
                            const uint8_t* input_buffer,
                            void *data,
                            int(*callback)(void*, const void*, size_t)) {
  BrotliMemIn Input(input_buffer, input_size);
  MyBrotliOut Output(data, callback);
  const BrotliParams *Params = reinterpret_cast<const BrotliParams*>(params);

  return BrotliCompress(*Params, &Input, &Output);
}

extern "C" RustBrotliCompressor*
RustBrotliCompressorCreate(const RustBrotliParams *params) {
  const BrotliParams *Params = reinterpret_cast<const BrotliParams*>(params);
  BrotliCompressor *Compressor = new BrotliCompressor(*Params);
  return reinterpret_cast<RustBrotliCompressor*>(Compressor);
}

extern "C" void
RustBrotliCompressorDestroy(RustBrotliCompressor* c) {
  delete reinterpret_cast<BrotliCompressor*>(c);
}

extern "C" size_t
RustBrotliCompressorInputBlockSize(const RustBrotliCompressor* c) {
  const BrotliCompressor *C = reinterpret_cast<const BrotliCompressor*>(c);
  return C->input_block_size();
}

extern "C" int
RustBrotliCompressorWriteMetaBlock(RustBrotliCompressor* c,
                                   size_t input_size,
                                   const uint8_t* input_buffer,
                                   int is_last,
                                   size_t *encoded_size,
                                   uint8_t *encoded_buffer) {
  BrotliCompressor *C = reinterpret_cast<BrotliCompressor*>(c);
  return C->WriteMetaBlock(input_size, input_buffer, is_last,
                           encoded_size, encoded_buffer);
}

extern "C" int
RustBrotliCompressorWriteMetadata(RustBrotliCompressor* c,
                                  size_t input_size,
                                  const uint8_t* input_buffer,
                                  int is_last,
                                  size_t *encoded_size,
                                  uint8_t *encoded_buffer) {
  BrotliCompressor *C = reinterpret_cast<BrotliCompressor*>(c);
  return C->WriteMetadata(input_size, input_buffer, is_last,
                          encoded_size, encoded_buffer);
}

extern "C" int
RustBrotliCompressorFinishStream(RustBrotliCompressor* c,
                                 size_t *encoded_size,
                                 uint8_t *encoded_buffer) {
  BrotliCompressor *C = reinterpret_cast<BrotliCompressor*>(c);
  return C->FinishStream(encoded_size, encoded_buffer);
}

extern "C" void
RustBrotliCompressorCopyInputToRingBuffer(RustBrotliCompressor* c,
                                          size_t input_size,
                                          const uint8_t *input_buffer) {
  BrotliCompressor *C = reinterpret_cast<BrotliCompressor*>(c);
  C->CopyInputToRingBuffer(input_size, input_buffer);
}

extern "C" int
RustBrotliCompressorWriteBrotliData(RustBrotliCompressor* c,
                                    int is_last,
                                    int force_flush,
                                    size_t *out_size,
                                    uint8_t **output) {
  BrotliCompressor *C = reinterpret_cast<BrotliCompressor*>(c);
  return C->WriteBrotliData(is_last, force_flush, out_size, output);
}
