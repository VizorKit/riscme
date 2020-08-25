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
    while(buffer != NULL && *buffer != '\0') {
        lexer_t lex = lexer_get(buffer, line, pos);
        if(lex.token.value == ENDLINE)
        {
            line++;
        }
        
        buffer += strlen(lex.token.data);
        lex_list.lexers[lex_list.size] = lex;
        lex_list.size++;
    }
    return lex_list;
}

token_t token_get(const char * buffer) {
    token_t token = {
        .value = EMPTY,
        .data = {'\0'},
    };
    char c = *buffer;
    switch (c)
    {
    case '#':
    {
        const char *comment = buffer;
        int length = 0;
        token.value = COMMENT;
        while (comment != NULL && *comment != 10 && *comment != '\0')
        {
            comment++;
            length++;
        }
        memcpy(token.data, buffer, sizeof(char) * length);
        break;
    }
    case ' ':
    {
        token.value = SEPARATOR;
        token.data[0] = ' ';
        break;
    }
    case '\t':
    {
        token.value = SEPARATOR;
        token.data[0] = '\t';
        break;
    }
    case '\n':
    {
        token.value = ENDLINE;
        token.data[0] = '\n';
        break;
    }
    case '\r':
    {
        token.value = ENDLINE;
        token.data[0] = '\r';
        break;
    }
    case ',':
    {
        token.value = SEPARATOR;
        token.data[0] = ',';
        break;
    }
    case '\0':
    {
        token.value = EMPTY;
        token.data[0] = '\0';
    }
    default:
    {
        const char *value = buffer;
        int length = 0;
        token.value = VALUE;
        while (*value != '\0' && *value != '\n' && *value != ',' && *value != ' ' && *value != '\t' && *value != '\r')
        {
            length++;
            value++;
        }
        memcpy(token.data, buffer, sizeof(char) * length);
        break;
    }
    }
    return token;
}

void lexer_free_list(lexer_l * lexers) {
    free(lexers->lexers);
}