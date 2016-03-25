#ifndef BROTLI_CAPI_H
#define BROTLI_CAPI_H

#if defined(__cplusplus) || defined(c_plusplus)
extern "C" {
#endif

struct BrotliCompress;
struct BrotliParams;
typedef struct BrotliCompress RustBrotliCompress;
typedef struct BrotliParams RustBrotliParams;

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
int RustBrotliCompressBuffer(RustBrotliParams *params,
                             size_t input_size,
                             const uint8_t* input_buffer,
                             size_t* encoded_size,
                             uint8_t* encoded_buffer);

#if defined(__cplusplus) || defined(c_plusplus)
}
#endif

#endif // BROTLI_CAPI_H
