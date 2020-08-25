#include "../src/lexer/lexer.h"
#include <assert.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

void test_multi() {
    printf("test_multi\n");
    /* 21 total */
    const char * buffer = " # This is a comment\n\t.globl label\nlabel:\nbneqz x0,x0,label\t\t # comment";
    
    // printf("%s\n", lexers[21].value);
    // assert(strcmp(lexers[21].value, "# comment") == 0);
    // assert(strcmp(lexers[4].value, ".globl") == 0);
    // assert(lexers[4].line == 2);
    // free(lexers);
}

void test_value() {
    printf("test_value\n");
    lexer_t t = process_tokenizer("bneqz", 1, 0);
    assert(strcmp(t.value, "bneqz") == 0);
    assert(t.token == VALUE);
    free(t.value);
}

void test_comment() {
    printf("test_comment\n");
    lexer_t t = process_tokenizer("# This is a comment\n", 1, 0);
    assert(t.token == COMMENT);
    assert(strcmp(t.value, "# This is a comment") == 0);
    free(t.value);
}

void test_separator() {
    printf("test_separator\n");
    lexer_t t = process_tokenizer("\t", 1, 0);
    assert(t.token == SEPARATOR);
    lexer_t t2 = process_tokenizer(" ", 1, 0);
    assert(t2.token == SEPARATOR);
    lexer_t t3 = process_tokenizer(",", 1, 0);
    assert(t3.token == SEPARATOR);
    free(t.value);
    free(t2.value);
    free(t3.value);
}

void test_endline() {
    printf("test_endline\n");
    lexer_t t = process_tokenizer("\n", 1, 0);
    assert(t.token == ENDLINE);
    assert(t.pos = 1);
    free(t.value);
}

int main() {
    test_multi();
    test_endline();
    test_separator();
    test_comment();
    test_value();
}