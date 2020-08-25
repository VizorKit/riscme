#include "../src/box/box.h"
#include <assert.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

void test_new() {
    printf("test_new\n");
    box_t * box = box_new();
    assert(box->cap == 20);
    assert(box->size == 0);
    box_free(box);
}

void test_add() {
    printf("test_add\n");
    box_t * box = box_new();
    item_t item = {
        .data = "my item",
        .data_size = sizeof(char) * 8,
    };
    int index = box_add(box, item);
    assert(index == 0);
    assert(strcmp(box->items[0].data, "my item") == 0);
    box_free(box);
}

void test_free() {
    printf("test_free\n");
    box_t * box = box_new();
    item_t item = {
        .data = "my item",
        .data_size = sizeof(char) * 8,
    };
    box_free(box);
    assert(1 == 1);
}
void test_resize() {
    printf("test_resize\n");
    box_t * box = box_new();
    for(int i = 0; i < INITIAL_BOX_CAP + 5; i++)
    {
        item_t item = {
            .data = "test",
            .data_size = sizeof(char) * 5
        };
        box_add(box, item);
    }
    box_free(box);
}

int main() {
    printf("box\n");
    test_new();
    test_add();
    test_free();
    test_resize();
}