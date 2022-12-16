#ifndef IP_STACK_H
#define IP_STACK_H

#include <stdlib.h>
#include <stdio.h>

/**
 * Defines an in-place stack implemented using arrays
*/
struct ip_stack
{
    int* content;
    unsigned int size;
    unsigned int _max_size;

} typedef ip_stack;

void _grow(ip_stack* s);
ip_stack get_empty(unsigned int max_size);

void push(ip_stack* s, int n);
int pop(ip_stack* s);

void free_stack(ip_stack* s);


#endif