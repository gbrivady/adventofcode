#include "ip_stack.h"

void _grow(ip_stack* s){
    s->content = realloc(s->content, 2*s->_max_size);
    s->_max_size *= 2;
}

ip_stack get_empty(int max_size){
    ip_stack s;
    s.content = malloc(max_size*sizeof(int));
    s._max_size = max_size;
    s.size = 0;
    return s;
}

void push(ip_stack* s, int n){
    if (s->size == s->_max_size)
        _grow(s);
    s->content[s->size++] = n;
}

int pop(ip_stack* s){
    if(s->size == 0){
        printf("Error : pop from empty stack");
        return -1;
    }
    return s->content[--s->size];
}

void free_stack(ip_stack* s){
    free(s->content);
}