#include "round2.h"
#include <stdio.h>

round2 make_round(char* line){

    round2 r = {.elf = 0, .result = 0};
    switch (line[0])
    {
    case 'A':
        r.elf = 1; //Rock
        break;
    case 'B':
        r.elf = 2; //Paper
        break;
    case 'C':
        r.elf = 3; //Scissor
        break;
    default:
        break;
    }
    switch (line[2])
    {
    case 'X':
        r.result = -1; //lose
        break;
    case 'Y':
        r.result = 0; //draw
        break;
    case 'Z':
        r.result = 1; //win
        break;
    default:
        break;
    }
    return r;
}

int round_score(round2 r){
    int elf = r.elf;
    int res  = r.result;
    printf("%d %d %d\n", elf, res, pos_mod(elf+res, 3));

    switch (res)
    {
    case -1:
        return 0+pos_mod(elf+res, 3);
    case 0:
        return 3+r.elf;
    case 1:
        return 6+pos_mod(elf+res, 3);
    default:
        return 0;
    }
}