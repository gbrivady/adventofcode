#include <stdio.h>

int contain(char* line){
    int min1, max1, min2, max2;
    sscanf(line, "%d-%d,%d-%d", &min1, &max1, &min2, &max2);

    if(min1 <= min2 && min2 <= max1)
        return 1;
    if (min2 <= min1 && max1 <= max2)
        return 1;
    if (min2 <= max1 && min2 >= min1)
        return 1;
    if (min1 <= max2 && min1 >= min2)
        return 1;
    if (max2 <= max1 && max2 >= min1)
        return 1;
    if (max1 <= max2 && max1 >= min2)
        return 1;
    return 0;
}

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }
    char buffer[100];
    int nb_pair = 0;

    while (fgets(buffer, 100, p_file))
    {
        nb_pair += contain(buffer);
    }
    
    printf("%d", nb_pair);
    return 0;
}
