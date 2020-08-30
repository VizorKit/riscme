#include "./lexer.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* information of result is included in lexer. */
/* if returned lexer is token ENDLINE increase the this_line variable */
lexer_t lexer_get(const char *buffer, int line, int pos)
{
    token_t token = token_get(buffer);
    lexer_t lexer = {
        .line = line,
        .pos = pos,
        .token = token};
    return lexer;
}

lexer_l lexer_get_list(const char * buffer) {
    lexer_l lex_list = {
        .lexers = malloc(sizeof(lexer_t) * 255),
        .size = 0,
        .cap = 255
    };
    int line = 0;
    int pos = 0;
    while(*buffer != '\0') {
        if(lex_list.size == lex_list.cap)
        {
            lex_list.cap = lex_list.cap << 1 ;
            lex_list.lexers = realloc(lex_list.lexers, sizeof(lexer_t) * lex_list.cap);
        }
        lexer_t lex = lexer_get(buffer, line, pos);
        if(lex.token.value == ENDLINE)
        {
            line++;
            pos = 0;
        }
        int length = strlen(lex.token.data);
        buffer += length;
        pos += length;
        lex_list.lexers[lex_list.size] = lex;
        lex_list.size++;
    }
    return lex_list;
}

token_t token_get(const char * buffer) {
    token_t token = {
        .value = EMPTY,
        .data = NULL,
    };
    int c = (int)*buffer;
    const char *buf_cpy = buffer;
    token.value = simple_get(c);
    if(token.value == COMMENT)
    {
        int length = 0;
        token.value = COMMENT;
        while (buffer != NULL && *buffer != 10 && *buffer != '\0')
        {
            length++;
            buffer++;
        }
        strncpy(token.data, buf_cpy, length);
    }
    else if(token.value == VALUE)
    {
        int length = 0;
        token.value = VALUE;
        while (simple_get(*buffer) == VALUE)
        {
            length++;
            buffer++;
        }
        strncpy(token.data, buf_cpy, length);
    }
    else {
        token.data = (char)c;
    }
    return token;
}

void lexer_free_list(lexer_l lexers) {
    free(lexers.lexers);
}