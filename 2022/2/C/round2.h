#ifndef ROUND2_H
#define ROUND2_H
#define pos_mod(i, n) ((i % n + n) % n)

struct round2
{
    int elf;
    int result;
} typedef round2;

round2 make_round(char* line);

int round_score(round2 r);


#endif