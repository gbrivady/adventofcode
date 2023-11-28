#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void check_cycle(int* nb, int* reg, int* sum){
    if ((*nb)%40 == 20 && *nb <= 220){
        *sum += (*nb) * (*reg);
    }
    if (*nb % 40 == 0){
        putc('\n', stdout);
        return;
    }
    putc((
        ((*nb)%40 == (*reg)) || 
        ((*nb)%40 == (*reg)+1) || 
        ((*nb)%40 == (*reg)+2)) ? '#' : '.', stdout);
    
}

void read_cycle(char* line, int* nb_cycles, int* reg, int* sum){
    if (strcmp(line, "noop\n") == 0){
        (*nb_cycles)++; //beginning of noop
        check_cycle(nb_cycles, reg, sum); //during noop
        return;
        
    }
    int val;
    sscanf(line, "addx %d", &val);
    (*nb_cycles)++; //beginning of addx
    check_cycle(nb_cycles, reg, sum); //during addx first part
    (*nb_cycles)++; //Begininning of addx part2
    check_cycle(nb_cycles, reg, sum); //during add 2nd part
    *reg += val; //end of addx
    // printf("%d %d \n", *nb_cycles, *reg);
}

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("../input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }
    int sum = 0;
    int reg = 1;
    int nb_cycles = 0;
    char buffer[100];
    while (fgets(buffer, 100, p_file)){
        read_cycle(buffer, &nb_cycles, &reg, &sum);
    }
    
    printf("%d %d", sum, reg);
    return 0;
}
