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
    node_t * second = dbl_link_add(&list, first, "2");
    node_t * third = dbl_link_add(&list, second, "3");
    assert(strcmp(third->prev->data, "2") == 0);
    assert(strcmp(first->next->data, "2") == 0);

    node_t * insert = dbl_link_insert(&list, first, "1.5");
    assert(strcmp(first->next->data, "1.5") == 0);
    assert(strcmp(insert->next->data, "2") == 0);
    assert(strcmp(second->prev->data, "1.5") == 0);
    dbl_link_free(&list);
}

int main() {
    printf("linked\n");
    test_create();
    test_add_insert();
}