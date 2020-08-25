#include "./box.h"
#include <stdlib.h>
#include <string.h>

void resize_box_if_full(box_t *box);
box_t * box_new()
{
    box_t box = {
        .items = malloc(sizeof(item_t) * INITIAL_BOX_CAP),
        .cap = INITIAL_BOX_CAP,
        .size = 0,
        .ospaces = {0},
        .ospaces_cnt = 0,
    };
    box_t * box_ptr = malloc(sizeof(box_t));
    memcpy(box_ptr, &box, sizeof(box_t));
    return box_ptr;
}

int box_add(box_t *box, item_t item)
{
    resize_box_if_full(box);
    memcpy(&box->items[box->size++], &item, sizeof(item_t));
    return box->size - 1;
}
item_t * box_get(box_t * box, int index) {
    if(index >= box->size) {
        return NULL;
    }
    return &box->items[index];
}
void box_free(box_t *box)
{
    free(box);
}
void resize_box_if_full(box_t *box)
{
    if (box->cap == box->size)
    {
        box->items = realloc(box->items, box->cap *= 2);
    }
}
