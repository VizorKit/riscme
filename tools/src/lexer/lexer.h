#pragma once

typedef enum _token_ {
    COMMENT,
    VALUE,
    SEPARATOR,
    UNDEFINED,
    ENDLINE
} TOKEN;

typedef struct Lexer {
    int line;
    char * value;
    int pos;
    TOKEN token;
} lexer_t;

lexer_t process_tokenizer(const char * buffer, int line, int pos);
lexer_t * process_all_tokens(const char * buffer);