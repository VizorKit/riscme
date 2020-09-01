#include "./format.h"
#include "../lexer/lexer.h"
#include "../linked/linked.h"

int is_separator(TOKEN_E token);
node_t * find_until_prev(node_t * node, int (*is_token)(TOKEN_E));
void format_lex_list(lexer_l * lex_list)
{
    dbl_link_l * list = lex_list->list;
    node_t * curr = dbl_link_get_first(list);
    int longest_value = 0;
    int comment_index = 99;
    int last_value = 99;
    int current_index = 0;
    node_t * line_start = curr;
    while(curr->next != NULL) {
        lexer_t * this_lex = (lexer_t *)curr->data;
        switch(this_lex->token.value)
        {
            case ENDLINE:
                line_start = curr->next;
                comment_index = 99;
                last_value = 99;
                current_index = 0;
                break;
            case COMMENT:
                //comment
                comment_index = current_index;
                if(last_value != 99)
                {
                    // this is a comment after a value
                }
                else
                {
                    // ensure that there are no trailing spaces or tabs
                    dbl_link_insert(lex_list->list, line_start, );
                }
                
                break;
            case VALUE:
                //value
                last_value = current_index;
                break;
            case TAB:
                //tab
                break;
            case SPACE:
                //space
                break;
            default:
                //do nothing
                break;
        }
        curr = curr->next;
        current_index++;
    }
    if(((lexer_t *)curr->data)->token.value != ENDLINE)
    {
        //add endline token;
        dbl_link_add(lex_list->list, curr, le)
    }
}
char * format_buffer(const char * to_format);
void format_file(const char * file_path);