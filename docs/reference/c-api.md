# Plasma C API Documentation

The Plasma C API provides a thread-safe interface for integrating Plasma cache into C/C++ applications and other toolchains.

## Installation

### From Releases

Download the pre-built libraries for your platform from the [releases page](https://github.com/plasmaorg/plasma/releases):

- **Linux**: `libplasma.so` + `plasma.h`
- **macOS**: `libplasma.dylib` + `plasma.h`
- **Windows**: `plasma.dll` + `plasma.h`

### Building from Source

```bash
# Clone the repository
git clone https://github.com/plasmaorg/plasma.git
cd plasma

# Build the library
cargo build --release --lib

# The library will be at:
# - Linux: target/release/libplasma.so
# - macOS: target/release/libplasma.dylib
# - Windows: target/release/plasma.dll

# The header file will be at:
# include/plasma.h
```

## Quick Start

### Basic Example

```c
#include <plasma.h>
#include <stdio.h>

int main() {
    // Initialize cache
    PlasmaCache *cache = plasma_cache_init("/tmp/my-cache");
    if (!cache) {
        fprintf(stderr, "Failed to init: %s\n", plasma_last_error());
        return 1;
    }

    // Store data
    const char *data = "Hello, World!";
    const char *hash = "abc123...";
    if (plasma_cache_put(cache, hash, (uint8_t*)data, 13) != PLASMA_OK) {
        fprintf(stderr, "Failed to put: %s\n", plasma_last_error());
    }

    // Retrieve data
    uint8_t buffer[1024];
    size_t bytes_read;
    if (plasma_cache_get(cache, hash, buffer, sizeof(buffer), &bytes_read) == PLASMA_OK) {
        printf("Retrieved %zu bytes\n", bytes_read);
    }

    // Cleanup
    plasma_cache_free(cache);
    return 0;
}
```

### Building Your Application

#### GCC/Clang

```bash
gcc -o myapp myapp.c -I/path/to/plasma/include -L/path/to/plasma/lib -lplasma
```

#### CMake

```cmake
cmake_minimum_required(VERSION 3.10)
project(MyApp)

# Find Plasma
find_library(PLASMA_LIB plasma PATHS /path/to/plasma/lib)
include_directories(/path/to/plasma/include)

add_executable(myapp myapp.c)
target_link_libraries(myapp ${PLASMA_LIB})
```

#### pkg-config

If Plasma is installed system-wide:

```bash
gcc -o myapp myapp.c $(pkg-config --cflags --libs plasma)
```

## API Reference

### Types

#### `PlasmaCache`

Opaque handle to a cache instance. Must be freed with `plasma_cache_free()`.

```c
typedef struct PlasmaCache PlasmaCache;
```

#### Error Codes

```c
#define PLASMA_OK                 0   // Success
#define PLASMA_ERROR             -1   // General error
#define PLASMA_ERROR_NOT_FOUND   -2   // Artifact not found
#define PLASMA_ERROR_INVALID_HASH -3  // Invalid hash format
#define PLASMA_ERROR_IO          -4   // I/O error
```

### Functions

#### `plasma_cache_init`

Initialize a new cache instance with default eviction settings (5GB max size, LFU policy, 7 days TTL).

```c
PlasmaCache* plasma_cache_init(const char *cache_dir);
```

**Parameters:**
- `cache_dir`: Path to cache directory (NULL-terminated C string)

**Returns:**
- Pointer to `PlasmaCache` on success
- `NULL` on error (use `plasma_last_error()` for details)

**Example:**
```c
PlasmaCache *cache = plasma_cache_init("/home/user/.cache/plasma");
if (!cache) {
    fprintf(stderr, "Init failed: %s\n", plasma_last_error());
}
```

---

#### `plasma_cache_init_with_eviction`

Initialize a new cache instance with custom eviction settings.

```c
PlasmaCache* plasma_cache_init_with_eviction(
    const char *cache_dir,
    uint64_t max_size_bytes,
    int eviction_policy,
    uint64_t ttl_seconds
);
```

**Parameters:**
- `cache_dir`: Path to cache directory (NULL-terminated C string)
- `max_size_bytes`: Maximum cache size in bytes (0 for default: 5GB)
- `eviction_policy`: Eviction policy (0=LRU, 1=LFU, 2=TTL)
- `ttl_seconds`: Default TTL in seconds (0 for default: 7 days)

**Returns:**
- Pointer to `PlasmaCache` on success
- `NULL` on error (use `plasma_last_error()` for details)

**Example:**
```c
// 10GB cache with LRU eviction and 14 day TTL
PlasmaCache *cache = plasma_cache_init_with_eviction(
    "/home/user/.cache/plasma",
    10ULL * 1024 * 1024 * 1024,  // 10GB
    0,                           // LRU
    14 * 24 * 60 * 60           // 14 days
);
if (!cache) {
    fprintf(stderr, "Init failed: %s\n", plasma_last_error());
}
```

**Eviction Policies:**
- `0` (LRU): Least Recently Used - evicts objects not accessed for longest time
- `1` (LFU): Least Frequently Used - evicts objects with lowest access count (default)
- `2` (TTL): Time To Live - evicts objects older than specified TTL

---

#### `plasma_cache_free`

Free a cache instance.

```c
void plasma_cache_free(PlasmaCache *cache);
```

**Parameters:**
- `cache`: Cache instance to free

**Notes:**
- Safe to call with `NULL`
- Must not use the cache after calling this function

---

#### `plasma_cache_put`

Store an artifact in the cache.

```c
int plasma_cache_put(
    PlasmaCache *cache,
    const char *hash,
    const uint8_t *data,
    size_t data_len
);
```

**Parameters:**
- `cache`: Cache instance
- `hash`: Content hash (SHA256, 64 hex characters)
- `data`: Data to store
- `data_len`: Length of data in bytes

**Returns:**
- `PLASMA_OK` on success
- Error code on failure

**Example:**
```c
const char *data = "artifact content";
const char *hash = "abc123def456...";
int result = plasma_cache_put(cache, hash, (uint8_t*)data, strlen(data));
if (result != PLASMA_OK) {
    fprintf(stderr, "Put failed: %s\n", plasma_last_error());
}
```

---

#### `plasma_cache_get`

Retrieve an artifact from the cache.

```c
int plasma_cache_get(
    PlasmaCache *cache,
    const char *hash,
    uint8_t *output_buffer,
    size_t buffer_size,
    size_t *bytes_written
);
```

**Parameters:**
- `cache`: Cache instance
- `hash`: Content hash to retrieve
- `output_buffer`: Buffer to write data (must be pre-allocated)
- `buffer_size`: Size of output buffer
- `bytes_written`: Output parameter for actual bytes written

**Returns:**
- `PLASMA_OK` on success
- `PLASMA_ERROR_NOT_FOUND` if artifact doesn't exist
- `PLASMA_ERROR` if buffer is too small or other error

**Example:**
```c
uint8_t buffer[4096];
size_t bytes_read;
int result = plasma_cache_get(cache, hash, buffer, sizeof(buffer), &bytes_read);
if (result == PLASMA_OK) {
    printf("Read %zu bytes\n", bytes_read);
} else if (result == PLASMA_ERROR_NOT_FOUND) {
    printf("Artifact not found\n");
}
```

---

#### `plasma_cache_exists`

Check if an artifact exists in the cache.

```c
int plasma_cache_exists(
    PlasmaCache *cache,
    const char *hash,
    int *exists
);
```

**Parameters:**
- `cache`: Cache instance
- `hash`: Content hash to check
- `exists`: Output parameter (1 if exists, 0 if not)

**Returns:**
- `PLASMA_OK` on success
- Error code on failure

**Example:**
```c
int exists;
if (plasma_cache_exists(cache, hash, &exists) == PLASMA_OK) {
    printf("Artifact %s\n", exists ? "exists" : "not found");
}
```

---

#### `plasma_cache_delete`

Delete an artifact from the cache.

```c
int plasma_cache_delete(
    PlasmaCache *cache,
    const char *hash
);
```

**Parameters:**
- `cache`: Cache instance
- `hash`: Content hash to delete

**Returns:**
- `PLASMA_OK` on success
- Error code on failure

**Example:**
```c
if (plasma_cache_delete(cache, hash) == PLASMA_OK) {
    printf("Artifact deleted\n");
}
```

---

#### `plasma_last_error`

Get the last error message for the current thread.

```c
const char* plasma_last_error(void);
```

**Returns:**
- Pointer to NULL-terminated error string
- `NULL` if no error

**Notes:**
- Error message is valid until next API call
- Do not free the returned pointer
- Thread-local (each thread has its own error)

---

#### `plasma_version`

Get the library version string.

```c
const char* plasma_version(void);
```

**Returns:**
- Pointer to version string (e.g., "0.8.1")
- String is statically allocated, do not free

---

## Error Handling

All functions follow a consistent error handling pattern:

```c
int result = plasma_cache_put(cache, hash, data, len);
if (result != PLASMA_OK) {
    // Get detailed error message
    const char *error = plasma_last_error();
    fprintf(stderr, "Operation failed: %s\n", error);

    // Check specific error codes
    if (result == PLASMA_ERROR_NOT_FOUND) {
        // Handle not found
    } else if (result == PLASMA_ERROR_IO) {
        // Handle I/O error
    }
}
```

## Thread Safety

The Plasma C API is **fully thread-safe**:

- Multiple threads can safely access the same cache instance
- Each thread has its own error state (`plasma_last_error()`)
- No global state or locks required by the caller

```c
// Thread-safe example
void* worker_thread(void *arg) {
    PlasmaCache *cache = (PlasmaCache*)arg;

    // Safe to call from multiple threads
    int exists;
    plasma_cache_exists(cache, "hash123", &exists);

    return NULL;
}
```

## Platform-Specific Notes

### Linux

- Library: `libplasma.so`
- Runtime library path: Set `LD_LIBRARY_PATH` or install to `/usr/lib`

```bash
export LD_LIBRARY_PATH=/path/to/plasma/lib:$LD_LIBRARY_PATH
./myapp
```

### macOS

- Library: `libplasma.dylib`
- Runtime library path: Set `DYLD_LIBRARY_PATH` or use `@rpath`

```bash
export DYLD_LIBRARY_PATH=/path/to/plasma/lib:$DYLD_LIBRARY_PATH
./myapp
```

### Windows

- Library: `plasma.dll`
- Place DLL in same directory as executable or in system PATH

## Examples

> **Note**: C API examples are coming soon.

## Integration Patterns

### Build System Integration

Example of integrating Plasma into a build system:

```c
#include <plasma.h>

void check_build_cache(const char *artifact_hash) {
    PlasmaCache *cache = plasma_cache_init(".build/cache");
    if (!cache) return;

    int exists;
    if (plasma_cache_exists(cache, artifact_hash, &exists) == PLASMA_OK) {
        if (exists) {
            // Skip rebuild, artifact is cached
            uint8_t buffer[1024*1024];  // 1MB buffer
            size_t bytes_read;
            plasma_cache_get(cache, artifact_hash, buffer, sizeof(buffer), &bytes_read);
            // Use cached artifact...
        }
    }

    plasma_cache_free(cache);
}
```

### Error Logging Wrapper

```c
#define PLASMA_CHECK(op, msg) \
    if ((op) != PLASMA_OK) { \
        log_error("%s: %s", msg, plasma_last_error()); \
        goto cleanup; \
    }

int build_with_cache() {
    PlasmaCache *cache = plasma_cache_init("/tmp/cache");
    if (!cache) return -1;

    PLASMA_CHECK(plasma_cache_put(cache, hash, data, len), "Failed to cache artifact");
    PLASMA_CHECK(plasma_cache_get(cache, hash, buf, sizeof(buf), &n), "Failed to retrieve");

cleanup:
    plasma_cache_free(cache);
    return 0;
}
```

## Troubleshooting

### Library Not Found

**Symptom:**
```
error while loading shared libraries: libplasma.so: cannot open shared object file
```

**Solution:**
- Set `LD_LIBRARY_PATH` (Linux) or `DYLD_LIBRARY_PATH` (macOS)
- Install library to system path
- Use `-rpath` linker flag

### Initialization Fails

**Symptom:**
```c
plasma_cache_init() returns NULL
```

**Common causes:**
- Invalid cache directory path
- Insufficient permissions
- Disk full

**Solution:**
```c
PlasmaCache *cache = plasma_cache_init("/tmp/cache");
if (!cache) {
    fprintf(stderr, "Init failed: %s\n", plasma_last_error());
    // Check permissions, disk space, path validity
}
```

### Buffer Too Small

**Symptom:**
```
plasma_cache_get() returns PLASMA_ERROR
plasma_last_error() says "Buffer too small"
```

**Solution:**
Use a larger buffer or dynamically allocate based on artifact size.

## Support

- **Documentation**: https://github.com/plasmaorg/plasma
- **Issues**: https://github.com/plasmaorg/plasma/issues
- **Discord**: https://discord.gg/tuist

## License

MPL-2.0 License - see [LICENSE](https://github.com/plasmaorg/plasma/blob/main/LICENSE.md) for details.
