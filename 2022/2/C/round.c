#include "round.h"
#include <stdio.h>

round make_round(char* line){

    round r = {.elf = 0, .me = 0};
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
        r.me = 1; //rock
        break;
    case 'Y':
        r.me = 2; //paper
        break;
    case 'Z':
        r.me = 3;
        break;
    default:
        break;
    }
    return r;
}

int round_score(round r){
    int elf = r.elf;
    if (r.elf == r.me)
    {
        return r.me+3;
    }
    if (r.elf == 1 && r.me == 2)
    {
        return r.me+6;
    }
    if (r.elf == 2 && r.me == 3)
    {
        return r.me+6;
    }
    if (r.elf == 3 && r.me == 1)
    {
        return r.me+6;
    }
    return r.me;
}