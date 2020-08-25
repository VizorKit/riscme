#include "../src/box/box.h"
#include <assert.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

void test_new() {
    box_t * box = box_new();
    assert(box->cap == 20);
    assert(box->size == 0);
}


int main() {
    test_new();
}