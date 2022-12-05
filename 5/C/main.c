#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../utils/ip_stack.h"

int read_stack_letter(char* buf, int offset){
    if(buf[offset] == '[')
        return buf[offset+1];
    return -1;
}

ip_stack* init_stacks(FILE* p_file){

    ip_stack* stack_array = malloc(9*sizeof(ip_stack));
    for (int i = 0; i < 9; i++)
        stack_array[i] = get_empty(64);

    char buffer0[100], buffer1[100], 
        buffer2[100], buffer3[100], 
        buffer4[100], buffer5[100], 
        buffer6[100], buffer7[100];

    char* buffers[8] = {buffer0, buffer1, buffer2, buffer3, buffer4, buffer5, buffer6, buffer7};
    for (int i = 0; i < 8; i++)
        fgets(buffers[i], 100, p_file);

    int letter;
    int offset;
    for (int i = 7; i >= 0; i--){
        for (int j = 0; j < 9; j++){
            letter = read_stack_letter(buffers[i], j*4);
            if (letter != -1)
                push(&(stack_array[j]), letter);            
        }
        
    }

    return stack_array;
}

void do_line1(char* line, ip_stack* stack_array){
    int nb, from, to;
    sscanf(line, "move %d from %d to %d", &nb, &from, &to);
    from--;
    to--;
    for (int i = 0; i < nb; i++)
        push(&(stack_array[to]), pop(&(stack_array[from])));
}

void do_line2(char* line, ip_stack* stack_array){
    int nb, from, to;
    sscanf(line, "move %d from %d to %d", &nb, &from, &to);
    from--;
    to--;
    int poped;
    ip_stack temp;
    int temp_cont[100] = {0};
    temp._max_size = 100;
    temp.content = temp_cont;
    temp.size = 0;
    for (int i = 0; i < nb; i++)
        push(&temp, pop(&(stack_array[from])));
    for (int i = 0; i < nb; i++){
        poped = temp.content[temp.size-1];
        temp.size--;
        push(&(stack_array[to]), poped);
    }
    
}

int main(int argc, char const *argv[])
{
    if (argc <= 1){
        printf("Provide a challenge number!");
        return 1;
    }
    int challenge_num = atoi(argv[1])-1;

    FILE* p_file = fopen("input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }

    ip_stack* stack_arr = init_stacks(p_file);

    // return 0;
    char buffer[100];   
    fgets(buffer, 100, p_file);
    fgets(buffer, 100, p_file);

    while (fgets(buffer, 100, p_file))
    {
        if(challenge_num)
            do_line2(buffer, stack_arr);
        else
            do_line1(buffer, stack_arr);

    }

    for (int i = 0; i < 9; i++)
    {
        stack_arr[i].size > 0 ? printf("%c", stack_arr[i].content[stack_arr[i].size-1]) : (printf(""));
        free_stack(&(stack_arr[i]));
    }
    free(stack_arr);
    return 0;
}
