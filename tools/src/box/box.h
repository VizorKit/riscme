#pragma once

#include <stdio.h>

#define INITIAL_BOX_CAP 20
#define MAX_GAP_SIZE 100

typedef struct Item {
    void * data;
    size_t data_size;
    int index;
} item_t;

typedef struct Box {
    item_t * items;
    int size;
    int cap;
    int ospaces[MAX_GAP_SIZE];
    int ospaces_cnt;
} box_t;

box_t * box_new();
/* returns the index of the added box */
int box_add(box_t * box, item_t item);
/* returns NULL if not found at index */
item_t * box_get(box_t * box, int index);
/* consumer must manage freeing of the data in items */
void box_free(box_t * box);