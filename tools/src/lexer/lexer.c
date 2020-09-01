#include "./lexer.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "../linked/linked.h"
#include "../debug/debug.h"

void resize_if_full(lexer_l *list);
/* information of result is included in lexer. */
/* if returned lexer is token ENDLINE increase the this_line variable */
lexer_t lexer_get(const char *buffer, int line, int pos, const char *buff_name)
{
    token_t token = token_get(buffer);
    lexer_t lexer = {
        .line = line,
        .pos = pos,
        .token = token,
        .file = buff_name};
    return lexer;
}

lexer_l lexer_get_list(const char *buffer, const char *buff_name)
{
    dbl_link_l list = dbl_link_new(255);
    lexer_l lex_list = {
        .lexers = malloc(sizeof(lexer_t) * 255),
        .size = 0,
        .cap = 255,
        .list = malloc(sizeof(dbl_link_l))};
    memcpy(lex_list.list, &list, sizeof(dbl_link_l));
    int line = 0;
    int pos = 0;
    node_t *curr;
    if (*buffer != '\0')
    {
        lexer_t lex = lexer_get(buffer, line, pos, buff_name);
        if (lex.token.value == ENDLINE)
        {
            line++;
            pos = 0;
        }
        int length = strlen(lex.token.data);
        buffer += length;
        pos += length;
        lex_list.lexers[lex_list.size] = lex;
        lex_list.size++;
        curr = dbl_link_add_first(&list, (void *)&lex_list.lexers[lex_list.size - 1]);
    }
    while (*buffer != '\0')
    {
        resize_if_full(&lex_list);
        lexer_t lex = lexer_get(buffer, line, pos, buff_name);
        if (lex.token.value == ENDLINE)
        {
            line++;
            pos = 0;
        }
        int length = strlen(lex.token.data);
        buffer += length;
        pos += length;
        lex_list.lexers[lex_list.size] = lex;
        lex_list.size++;
        dbl_link_add(&list, curr, (void *)&lex_list.lexers[lex_list.size - 1]);
    }
    return lex_list;
}

token_t token_get(const char *buffer)
{
    token_t token = {
        .value = EMPTY,
        .data = {'\0'},
    };
    int c = (int)*buffer;
    const char *buf_cpy = buffer;
    token.value = simple_get(c);
    if (token.value == COMMENT)
    {
        int length = 0;
        token.value = COMMENT;
        while (buffer != NULL && *buffer != 10 && *buffer != '\0')
        {
            length++;
            buffer++;
        }
        memcpy(token.data, buf_cpy, sizeof(char) * length);
    }
    else if (token.value == VALUE)
    {
        int length = 0;
        token.value = VALUE;
        while (simple_get(*buffer) == VALUE)
        {
            length++;
            buffer++;
        }
        memcpy(token.data, buf_cpy, sizeof(char) * length);
    }
    else
    {
        token.data[0] = (char)c;
    }
    return token;
}

void lexer_l_add(lexer_l * list, node_t * curr, lexer_t) {
    resize_if_full(list);
    list->
}

void lexer_free_list(lexer_l *lex_list)
{
    free(lex_list->list);
    free(lex_list->lexers);
}

void resize_if_full(lexer_l *list)
{
    if (list->size == list->cap)
    {
        list->cap = list->cap << 1;
        list->lexers = realloc(list->lexers, sizeof(lexer_t) * list->cap);
    }
}
