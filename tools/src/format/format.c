#include "./format.h"
#include "../lexer/lexer.h"
#include "../linked/linked.h"

void format_lex_list(lexer_l * lex_list)
{
    dbl_link_l * list = lex_list->list;
    node_t * curr = dbl_link_get_first(list);
    while(curr->next != NULL) {
        lexer_t * this_lex = (lexer_t *)curr->data;
        switch(this_lex->token.value)
        {
            case ENDLINE:
                //reset everything
                break;
            case COMMENT:
                //comment
                break;
            case VALUE:
                //value
                break;
            default:
                //do nothing
                break;
        }
        if(this_lex->token.value == ENDLINE) {
            // reset everything and start processing next line
        }
        curr = curr->next;
    }
}
char * format_buffer(const char * to_format);
void format_file(const char * file_path);