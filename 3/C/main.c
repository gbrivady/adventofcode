#include <stdio.h>

int get_index(char c){
    if (c >= 'a')
    {
        return c-'a';
    }
    return c-'A'+26;
}

struct char_count
{
    int count_1;
    int count_2;
} typedef char_count;

struct char_count2
{
    int count_1;
    int count_2;
    int count_3;
} typedef char_count2;

int get_sack_length(char* line){
    int i = 0;
    while (line[i] != '\n')
    {
        i++;
    }
    printf("%d ", i);
    return i;
}

int count_chars(char* line){
    char_count count[52] = {0};
    int len = get_sack_length(line);
    // printf("%d \n", len/2);
    for (int i = 0; i < len/2; i++)
    {
        count[get_index(line[i])].count_1 ++;
    }
    // printf("%c", line[len/2]);
    for (int i = len/2; i < len; i++)
    {
        count[get_index(line[i])].count_2 ++;
    }
    for (int i = 0; i < 52; i++)
    {
        if (count[i].count_1 >0 && count[i].count_2 >0)
        {
            // printf("%d\n", i+1);
            return i+1;
        }
        
    }
    return 0;
}

int get_group_prio(char** lines){
    int count[52][3] = {0};
    int j;
    for (size_t i = 0; i < 3; i++)
    {
        j = 0;
        while (lines[i][j] != '\n')
        {
            count[get_index(lines[i][j])][i] ++;
            j++;
        }
        
    }
    for (int i = 0; i < 52; i++)
    {
        if (count[i][0] >0 && count[i][1] >0 && count[i][2] > 0)
        {
            // printf("%d\n", i+1);
            return i+1;
        }
        
    }
}

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("input", "r");
    if (p_file == NULL)
    {
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }
    // printf("%d\n", get_index('p'));
    char* buffers[3];
    char buffer1[100];
    char buffer2[100];
    char buffer3[100];
    int sum = 0;
    while (fgets(buffer1, 100, p_file))
    {
        fgets(buffer2, 100, p_file);
        fgets(buffer3, 100, p_file);
        buffers[0] = buffer1;
        buffers[1] = buffer2;
        buffers[2] = buffer3;
        sum += get_group_prio(buffers);

    }
    printf("%d", sum);
    return 0;
}
