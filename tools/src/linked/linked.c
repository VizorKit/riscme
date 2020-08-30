#include "./linked.h"
#include <stdlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "../debug/debug.h"

void resize_if_full(dbl_link_l * list);
dbl_link_l dbl_link_new(const void * data, const int initial_cap) {
    assert(initial_cap >= 1);
    node_t node = {
        .prev = NULL,
        .data = data,
        .next = NULL
    };
    dbl_link_l linked = {
        .nodes = malloc(sizeof(node_t) * initial_cap),
        .first_node = NULL,
        .size = 1,
        .capacity = initial_cap,
    };
    memcpy(&linked.nodes[0], &node, sizeof(node_t));
    linked.first_node = &linked.nodes[0];
    return linked;
}

node_t * dbl_link_get_first(dbl_link_l * linked) {
    return linked->first_node;
}
node_t * dbl_link_add(dbl_link_l * list, node_t * prev, const void * data)
{
    resize_if_full(list);
    node_t node = {
        .prev = prev,
        .next = NULL,
        .data = data
    };
    memcpy(&(list->nodes[list->size]), &node, sizeof(node_t));
    debug_print("list->size {%d}\n", list->size);
    // I can access prev->next no problem.
    int thing = (int)prev->next;
    // I can assign node temp and access its data
    node_t * node_temp = &(list->nodes[list->size]);
    debug_print("%s\n", node_temp->data);
    // I can't assign the next node, to the node in the list.
    prev->next = &(list->nodes[list->size]);
    debug_print("%s\n", "kaboom");
    list->size++;
    return &list->nodes[list->size - 1];
}
node_t * dbl_link_insert(dbl_link_l * list, node_t * prev, const void * data)
{
    resize_if_full(list);
    node_t node = {
        .prev = prev,
        .next = prev->next,
        .data = data
    };
    memcpy(&list->nodes[list->size++], &node, sizeof(node_t));
    prev->next = &list->nodes[list->size - 1];
    return &list->nodes[list->size - 1];
}

void resize_if_full(dbl_link_l * list) {
    if (list->capacity == list->size) {
        list->capacity = list->capacity << 1;
        list->nodes = realloc(list->nodes, list->capacity * sizeof(node_t));
    }
}
