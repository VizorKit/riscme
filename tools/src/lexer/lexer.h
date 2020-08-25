#pragma once

typedef enum _token_
{
    COMMENT,
    VALUE,
    SEPARATOR,
    EMPTY,
    ENDLINE
} TOKEN_E;

typedef struct Token
{
    TOKEN_E value;
    char data[255];
} token_t;

typedef struct Lexer
{
    int line;
    int pos;
    token_t token;
} lexer_t;

typedef struct Lexers
{
    lexer_t * lexers;
    int size;
    int cap;
} lexer_l;

token_t token_get(const char *buffer);

lexer_t lexer_get(const char *buffer, int line, int pos);
lexer_l lexer_get_list(const char *buffer);
void lexer_free_list(lexer_l * lexers);
