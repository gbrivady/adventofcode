#include <stdio.h>
#include "elf.h"

int main(int argc, char const *argv[])
{

    FILE* p_file = fopen("./input", "r");
    if (p_file == NULL)
    {
        printf("Input file empty. Check if it exists.\n");
        return 1;
    }

    elf top1_elf = {.elf_number = -1, .cal = 0};
    elf top2_elf = {.elf_number = -1, .cal = 0};
    elf top3_elf = {.elf_number = -1, .cal = 0};
    elf new_elf = {.elf_number = -1, .cal = 0};

    char stop_char = getc(p_file);

    int cur_elf = 0;

    while (stop_char != EOF){

        ungetc(stop_char, p_file);

        new_elf = make_elf(p_file);
        new_elf.elf_number = cur_elf;

        rerank_elves(&top1_elf, &top2_elf, &top3_elf, &new_elf);

        cur_elf++;

        stop_char = getc(p_file);
    }

    printf("The elf with the most calories is the %d-th one, with %d calories;\n", top1_elf.elf_number, top1_elf.cal);
    printf("The elf with the 2nd most calories is the %d-th one, with %d calories;\n", top2_elf.elf_number, top2_elf.cal);
    printf("The elf with the 3rd most calories is the %d-th one, with %d calories;\n", top3_elf.elf_number, top3_elf.cal);
    printf("They carry a total of %d calories\n", top1_elf.cal+top2_elf.cal+top3_elf.cal);

    fclose(p_file);
    return 0;
}