#ifndef BROTLI_CAPI_H
#define BROTLI_CAPI_H

#if defined(__cplusplus) || defined(c_plusplus)
extern "C" {
#endif

struct BrotliCompressorOpaque;
struct BrotliParamsOpaque;
typedef struct BrotliCompressorOpaque RustBrotliCompressor;
typedef struct BrotliParamsOpaque RustBrotliParams;

typedef enum {
    RUST_MODE_GENERIC,
    RUST_MODE_TEXT,
    RUST_MODE_FONT = 2
} RustBrotliMode;

RustBrotliParams* RustBrotliParamsCreate(void);
void RustBrotliParamsDestroy(RustBrotliParams *params);
void RustBrotliParamsSetMode(RustBrotliParams *params,
                             RustBrotliMode mode);
void RustBrotliParamsSetQuality(RustBrotliParams *params, int quality);
void RustBrotliParamsSetLgwin(RustBrotliParams *params, int lgwin);
void RustBrotliParamsSetLgblock(RustBrotliParams *params, int lgblock);
void RustBrotliParamsSetEnableDictionary(RustBrotliParams *params, int enable);
void RustBrotliParamsSetEnableTransforms(RustBrotliParams *params, int enable);
void RustBrotliParamsSetGreedyBlockSplit(RustBrotliParams *params, int split);
void RustBrotliParamsSetEnableContextModeling(RustBrotliParams *params, int enable);
int RustBrotliCompressBuffer(const RustBrotliParams *params,
                             size_t input_size,
                             const uint8_t* input_buffer,
                             size_t* encoded_size,
                             uint8_t* encoded_buffer);
int RustBrotliCompressBufferVec(const RustBrotliParams *params,
                                size_t input_size,
                                const uint8_t* input_buffer,
                                void *data,
                                int(*callback)(void*, const void*, size_t));

RustBrotliCompressor* RustBrotliCompressorCreate(const RustBrotliParams *);
void RustBrotliCompressorDestroy(RustBrotliCompressor* c);
size_t RustBrotliCompressorInputBlockSize(const RustBrotliCompressor* c);
int RustBrotliCompressorWriteMetaBlock(RustBrotliCompressor* c,
                                       size_t input_size,
                                       const uint8_t* input_buffer,
                                       int is_last,
                                       size_t *encoded_size,
                                       uint8_t *encoded_buffer);
int RustBrotliCompressorWriteMetadata(RustBrotliCompressor* c,
                                      size_t input_size,
                                      const uint8_t* input_buffer,
                                      int is_last,
                                      size_t *encoded_size,
                                      uint8_t *encoded_buffer);
int RustBrotliCompressorFinishStream(RustBrotliCompressor* c,
                                     size_t *encoded_size,
                                     uint8_t *encoded_buffer);
void RustBrotliCompressorCopyInputToRingBuffer(RustBrotliCompressor* c,
                                               size_t input_size,
                                               const uint8_t *input_buffer);
int RustBrotliCompressorWriteBrotliData(RustBrotliCompressor* c,
                                        int is_last,
                                        int force_flush,
                                        size_t *out_size,
                                        uint8_t **output);

#if defined(__cplusplus) || defined(c_plusplus)
}
#endif

#endif // BROTLI_CAPI_H
