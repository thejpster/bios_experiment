#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum ErrorCode {
  Success,
  BadArgument,
};

struct BiosStrSlice {
  const uint8_t *ptr;
  uintptr_t length;
};

/**
 * Tells you about a serial port
 */
struct SerialInfo {
  uint8_t name[32];
  uint32_t baud;
};

/**
 * Arguments for the BiosApi::exec call.
 */
enum BiosArgs_Tag {
  Print,
  SerialGetInfo,
  Exit,
};

struct SerialGetInfo_Body {
  uint8_t idx;
  struct SerialInfo *out;
};

struct BiosArgs {
  enum BiosArgs_Tag tag;
  union {
    struct {
      struct BiosStrSlice print;
    };
    struct SerialGetInfo_Body serial_get_info;
  };
};

/**
 * Entry point to the BIOS
 */
struct BiosApi {
  enum ErrorCode (*exec)(struct BiosArgs*);
};
