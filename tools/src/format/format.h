#pragma once
#include "../lexer/lexer.h"

void format_lex_list(lexer_l * lex_list);
char * format_buffer(const char * to_format);
void format_file(const char * file_path);