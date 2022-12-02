#include <stdio.h>
#include "round2.h"

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("input", "r");
    if (p_file == NULL)
    {
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }

    char line[100];
    int nb_line = 0;
    int tot_score = 0;
    while (fgets(line, 100, p_file))
    {
        tot_score += round_score(make_round(line));
    }
    printf("%d", tot_score);
    return 0;
}
