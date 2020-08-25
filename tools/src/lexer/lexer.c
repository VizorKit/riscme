#include "./lexer.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* information of result is included in lexer. */
/* if returned lexer is token ENDLINE increase the this_line variable */
lexer_t process_tokenizer(const char *buffer, int line, int pos)
{
    lexer_t lexer = {
        .line = line,
        .pos = pos,
        .token = UNDEFINED,
        .value = malloc(sizeof(char) * 255)};
    char c = *buffer;
    switch (c)
    {
    case '#':
    {
        const char *comment = buffer;
        lexer.token = COMMENT;
        while (comment != NULL && *comment != 10)
        {
            lexer.pos++;
            comment++;
        }
        strncpy(lexer.value, buffer, ((lexer.pos < 255) ? lexer.pos : 255) - pos);
        break;
    }
    case ' ':
    {
        lexer.pos++;
        lexer.token = SEPARATOR;
        lexer.value = " ";
        break;
    }
    case '\t':
    {
        lexer.pos += 1;
        lexer.token = SEPARATOR;
        lexer.value = "\t";
        break;
    }
    case '\n':
    {
        lexer.pos++;
        lexer.token = ENDLINE;
        break;
    }
    case '\r':
    {
        lexer.pos++;
        lexer.token = ENDLINE;
        break;
    }
    case ',':
    {
        lexer.pos++;
        lexer.token = SEPARATOR;
        lexer.value = ",";
        break;
    }
    default:
    {
        const char *value = buffer;
        lexer.token = VALUE;
        while (*value != '\0' && *value != '\n' && *value != ',' && *value != ' ' && *value != '\t' && *value != '\r')
        {
            lexer.pos++;
            value++;
        }
        strncpy(lexer.value, buffer, ((lexer.pos < 255) ? lexer.pos : 255) - pos);
        break;
    }
    }
    return lexer;
}

lexer_t * process_all_tokens(const char * buffer) {
    lexer_t * lexers = malloc(sizeof(lexer_t *) * 22);
    int index = 0;
    int stop = 0;
    int line = 1;
    int lexer_pos = 0;
    int prev_lexer_pos = 0;
    int iter_pos = 0;
    while(*buffer != '\0' && buffer != NULL && stop != 1)
    {
        lexers[index] = process_tokenizer(buffer, line, lexer_pos);
        printf("lexers[index].value %s .pos %d\n", lexers[index].value, lexers[index].pos);
        if(lexers[index].token == ENDLINE)
        {
            line++;
            lexer_pos = 0;
        }
        iter_pos = prev_lexer_pos - lexer_pos;
        buffer += lexers[index].pos;
        index++;
    }
}