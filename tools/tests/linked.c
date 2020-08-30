#include "../src/linked/linked.h"
#include "../src/debug/debug.h"
#include <assert.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

void test_create() {
    printf("test_create\n");
    char * data = malloc(sizeof(char) * 8);
    data = "12345678";
    dbl_link_l list = dbl_link_new(data, 1);
    assert(list.first_node->prev == NULL);
    assert(list.first_node->next == NULL);
    assert(list.nodes[0].prev == NULL);
    assert(list.nodes[0].next == NULL);
    assert(strcmp("12345678", list.first_node->data) == 0);
    assert(strcmp("12345678", list.nodes[0].data) == 0);
}

void test_add_insert() {
    printf("test_add_insert\n");
    char * data = malloc(sizeof(char) * 8);
    data = "12345678";
    dbl_link_l list = dbl_link_new(data, 1);
    node_t * first = dbl_link_get_first(&list);
    assert(first != NULL);
    node_t * second = dbl_link_add(&list, first->prev, "2");
    node_t * third = dbl_link_add(&list, second->prev, "3");
    while(first->next != NULL) {
        first = first->next;
    }
    assert(strcmp(third->data, "3") == 0);
    assert(strcmp(first->data, "3") == 0);
}

int main() {
    printf("linked\n");
    test_create();
    test_add_insert();
}