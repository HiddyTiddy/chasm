#ifndef CHASM_LIB
#define CHASM_LIB

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace chasm {

struct TranslationOutput;

extern "C" {

/// takes an assembly-c_str and parses it.
/// if parsing fails, `parse_asm` will set `*err` and return `nullptr`
/// else `parse_asm` will return a pointer to an intermediate representation of the assembled instructions
///
/// if any of the pointers supplied are invalid or nullptr, the function may crash
///
/// ## `*err`
/// err can be
/// - `-1`: Statement Syntax Error
/// - `-2`: Label Syntax Error
/// - `-3`: Unexpected Token
/// - `-4`: Label Not Resolved Error
/// - `-5`: Cannot set location
const TranslationOutput *parse_asm(const char *assembly,
                                   int32_t *err);

const TranslationOutput *parse_asm_extend(const char *assembly,
                                          const TranslationOutput *previous,
                                          int32_t *err);

/// takes a pointer to the intermediate representation struct and links it.
/// if parsing fails, `parse_asm` will set `*err` and return `nullptr`
/// else `parse_asm` will return a pointer to an array of assembled instructions of length `len`
///
/// if any of the pointers supplied are invalid or nullptr, the function may crash
///
/// ## `*err`
/// err can be
/// - `-1`: Statement Syntax Error
/// - `-2`: Label Syntax Error
/// - `-3`: Unexpected Token
/// - `-4`: Label Not Resolved Error
/// - `-5`: Cannot set location
const uint16_t *link_asm(const TranslationOutput *translation, int32_t *err, uintptr_t *len);

} // extern "C"

} // namespace chasm

#endif // CHASM_LIB
