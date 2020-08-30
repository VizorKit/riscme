#include "../src/lexer/lexer.h"
#include <assert.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

void test_multi() {
    printf("test_multi\n");
    /* 21 total */
    const char * buffer = " # This is a comment\n\t.globl label\nlabel:\nbneqz x0,x0,label\t\t # comment";
    lexer_l lexes = lexer_get_list(buffer);
    assert(lexes.size == 23);
    assert(lexes.lexers[lexes.size - 1].token.value == COMMENT);
    lexer_free_list(lexes);
}

void test_value() {
    printf("test_value\n");
    lexer_t l = lexer_get("bneqz", 1, 0);
    assert(strcmp(l.token.data, "bneqz") == 0);
    assert(l.token.value == VALUE);
}

void test_comment() {
    printf("test_comment\n");
    lexer_t l = lexer_get("# This is a comment\n", 1, 0);
    assert(l.token.value == COMMENT);
    assert(strcmp(l.token.data, "# This is a comment") == 0);
}

void test_separator() {
    printf("test_separator\n");
    lexer_t l = lexer_get("\t", 1, 0);
    assert(l.token.value == TAB);
    lexer_t l2 = lexer_get(" ", 1, 0);
    assert(l2.token.value == SPACE);
    lexer_t l3 = lexer_get(",", 1, 0);
    assert(l3.token.value == COMMA);
}

void test_endline() {
    printf("test_endline\n");
    lexer_t l = lexer_get("\n", 1, 0);
    assert(l.token.value == ENDLINE);
    assert(l.pos = 1);
}

int main() {
    printf("lexer\n");
    test_endline();
    test_separator();
    test_comment();
    test_value();
    test_multi();
}