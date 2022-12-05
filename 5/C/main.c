#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct stack
{
    int* stack_content;
    int end;
} typedef stack;

int read_stack_letter(char* buf, int offset){
    if(buf[offset] == '[')
        return buf[offset+1];
    return -1;
}

stack* read_stacks(FILE* p_file){

    stack* stack_array = malloc(9*sizeof(stack));
    for (int i = 0; i < 9; i++)
    {
        stack_array[i].stack_content = malloc(8*9*sizeof(int));
        stack_array[i].end = 0;
    }
    
    char buffer0[100];    
    char buffer1[100];    
    char buffer2[100];    
    char buffer3[100];    
    char buffer4[100];    
    char buffer5[100];    
    char buffer6[100];
    char buffer7[100];
    char* buffers[8] = {buffer0, buffer1, buffer2, buffer3, buffer4, buffer5, buffer6, buffer7};
    for (int i = 0; i < 8; i++)
    {
        fgets(buffers[i], 100, p_file);
    }
    int letter;
    int offset;
    for (int i = 7; i >= 0; i--)
    {
        offset = 0;
        for (int j = 0; j < 9; j++)
        {
            letter = read_stack_letter(buffers[i], offset);
            if (letter != -1)
            {
                stack_array[j].stack_content[stack_array[j].end] = letter;
                stack_array[j].end += 1;
            }
            offset += 4;
            
        }
        
    }

    return stack_array;
}

void do_line(char* line, stack* stack_array){
    int nb, from, to;
    sscanf(line, "move %d from %d to %d", &nb, &from, &to);
    from--;
    to--;
    int poped;
    stack temp;
    int temp_cont[100] = {0};
    temp.stack_content = temp_cont;
    temp.end = 0;
    for (int i = 0; i < nb; i++)
    {
        poped = stack_array[from].stack_content[stack_array[from].end-1];
        stack_array[from].end--;
        temp.stack_content[temp.end] = poped;
        temp.end++;
    }
    for (int i = 0; i < nb; i++)
    {
        poped = temp.stack_content[temp.end-1];
        temp.end--;
        stack_array[to].stack_content[stack_array[to].end] = poped;
        stack_array[to].end++;
    }
    
}

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }

    stack* stack_arr = read_stacks(p_file);
    for (int i = 0; i < 9; i++)
    {
        printf("%c ", stack_arr[i].stack_content[stack_arr[i].end-1]);
    }
    
    char buffer[100];   
    fgets(buffer, 100, p_file);
    fgets(buffer, 100, p_file);

    while (fgets(buffer, 100, p_file))
    {
        do_line(buffer, stack_arr);
        for (int i = 0; i < 9; i++)
        {
            printf("%c ", stack_arr[i].stack_content[stack_arr[i].end-1]);
        }
    }

    for (int i = 0; i < 9; i++)
    {
        stack_arr[i].end > 0 ? printf("%c", stack_arr[i].stack_content[stack_arr[i].end-1]) : (printf(""));
    }
    
    return 0;
}
