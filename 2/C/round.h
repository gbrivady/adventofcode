#ifndef ROUND_H
#define ROUND_H

struct round
{
    int elf;
    int me;
} typedef round;

round make_round(char* line);

int round_score(round r);


#endif