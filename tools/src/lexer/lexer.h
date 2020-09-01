#pragma once
#include "../linked/linked.h"

typedef enum _simple_ {
    VALUE,
    SPACE,
    COMMA,
    ENDLINE,
    COMMENT,    // '#'
    TAB,
    EMPTY,  // '\0'
    BANG,   // '!'
    REL,    // '%'
    QUOTE,  // ''' '"'
    OPAREN,
    CPAREN,
    DOT,    // '.'
    REF,    // '\'
    COLON,
    INVALID,
    DIRECTIVE,
    OPCODE,
    IMMEDIATE,
    REGISTER,
    LABEL
} TOKEN_E;

typedef struct Token
{
    TOKEN_E value;
    char data[80];
} token_t;

typedef struct Lexer
{
    int line;
    int pos;
    token_t token;
    const char * file;
} lexer_t;

typedef struct Lexers
{
    lexer_t * lexers;
    int size;
    int cap;
    dbl_link_l * list;
} lexer_l;

token_t token_get(const char *buffer);

lexer_t lexer_get(const char *buffer, int line, int pos, const char * buff_name);
lexer_l lexer_get_list(const char *buffer, const char * buff_name);
void lexer_free_list(lexer_l * lex_list);
TOKEN_E simple_get(const char c);
void lexer_l_add(lexer_l * list, node_t * node);