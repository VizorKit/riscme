#include <stdint.h>
#include "./lexer.h"

const static enum _simple_ lookup[128] = {
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    TAB,
    ENDLINE, // '10'
    EMPTY,
    EMPTY,
    ENDLINE,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    EMPTY,
    SPACE,      // 32
    BANG,
    QUOTE,
    COMMENT,    // 35
    VALUE,
    REL,        // 37 
    VALUE,
    QUOTE,
    OPAREN,
    CPAREN,     // 41
    VALUE,
    VALUE,
    COMMA,      // 44
    VALUE,
    DOT,
    INVALID,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    COLON,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    INVALID,
    REF,
    INVALID,
    INVALID,
    VALUE,
    INVALID,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    VALUE,
    INVALID,
    INVALID,
    INVALID,
    INVALID    
};

TOKEN_E simple_get(const char c) {
    return lookup[(int)c];
}