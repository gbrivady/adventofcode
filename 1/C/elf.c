#include "elf.h"

int compare_elves(elf galadriel, elf legolas){
    if (galadriel.cal > legolas.cal)
    {
        return 0;
    }
    return 1;
}

elf make_elf(FILE* p_file){
    elf legolas = {.elf_number = 0, .cal = 0};
    int num;    
    char stop_char = getc(p_file);

    while (stop_char != '\n' && stop_char != EOF)
    {
        ungetc(stop_char, p_file);
        
        fscanf(p_file ,"%d", &num);
        legolas.cal += num;

        getc(p_file);
        stop_char = getc(p_file);
    }

    return legolas;
}

void rerank_elves(elf* p_first, elf* p_second, elf* p_third, elf* p_newbie){
    //Suppose the first 3 ones are correctly ranked
    if(compare_elves(*p_first, *p_newbie)){
        *p_third = *p_second;
        *p_second = *p_first;
        *p_first = *p_newbie;
    }
    else if(compare_elves(*p_second, *p_newbie)){
        *p_third = *p_second;
        *p_second = *p_newbie;
    }
    else if(compare_elves(*p_third, *p_newbie)){
        *p_third = *p_newbie;
    }
}