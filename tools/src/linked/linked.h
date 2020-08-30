#pragma once
#include <stdio.h>

struct Node {
    struct Node * prev;
    const void * data;
    struct Node * next;    
};

typedef struct Node node_t;
/* TODO:: benchmark this against SOA impl */
typedef struct DoubleLinkedList {
    node_t * nodes;
    node_t * first_node;
    int size;
    int capacity;
} dbl_link_l;

dbl_link_l dbl_link_new(const void * data, const int initial_cap);
/* will insert a new node after the current, and adjusting the next node to be after */
node_t * dbl_link_insert(dbl_link_l * list, node_t * current, const void * data);
/* returns the first one, so you can iterate over. */
node_t * dbl_link_get_first(dbl_link_l * linked);
/* will add a new node, this will overwrite the next node if not put at the end of the list */
node_t * dbl_link_add(dbl_link_l * list, node_t * current, const void * data);

void dbl_link_free(dbl_link_l * list);
