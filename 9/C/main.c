#include <stdio.h>
#include <stdlib.h>

#define GRID_SIZE 4096

void move_tail(int* tail, int* head){
    //where is the tail
    int rel_i = head[0]-tail[0];
    int rel_j = head[1]-tail[1];
    if (abs(rel_i) <= 1 && abs(rel_j) <= 1){
        return;
    }
    if (rel_j < 0){
        tail[1]--;
    } else if(rel_j > 0){
        tail[1]++;
    }
    if(rel_i < 0){
        tail[0]--;
    } else if(rel_i > 0){
        tail[0]++;
    }
    return;
}

void process_movement(int** tails, int* head, int** grid, char move, int amp, int* nb_pos_ht, int* nb_pos_tt){
    int move_i = 0;
    int move_j = 0;
    switch (move){
    case 'U':
        move_i = -1;
        break;
    case 'D':
        move_i = +1;
        break;
    case 'L':
        move_j = -1;
        break;
    case 'R':
        move_j = +1;
        break;
    default:
        break;
    }
    for (int k = 0; k < amp; k++)
    {
        head[0] += move_i;
        head[1] += move_j;
        move_tail(tails[0], head);
        for (int i = 0; i < 8; i++)
        {
            move_tail(tails[i+1], tails[i]);
        }
        if (grid[tails[0][0]][tails[0][1]]/10 == 0)
        {
            grid[tails[0][0]][tails[0][1]] += 10;
            (*nb_pos_ht)++;
        }
        
        if (grid[tails[8][0]][tails[8][1]]%10 == 0){
            grid[tails[8][0]][tails[8][1]]++;
            (*nb_pos_tt)++;
        }
        
    }
    
}

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("../input", "r");
    if (p_file == NULL)
    {
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }

    int** grid = calloc(GRID_SIZE, sizeof(int*));
    for (int i = 0; i < GRID_SIZE; i++)
    {
        grid[i] = calloc(GRID_SIZE, sizeof(int));
        for (int j = 0; j < GRID_SIZE; j++)
        {
            grid[i][j]=0;
        }
        
    }
    
    int* head = malloc(sizeof(int)*2);
    int** tails = calloc(9, sizeof(int*));
    for (int i = 0; i < 9; i++)
    {
        tails[i] = calloc(2, sizeof(int));
        tails[i][0] = GRID_SIZE/2;
        tails[i][1] = GRID_SIZE/2;
    }
    
    int nb_pos_ht = 1; //head of tail
    int nb_pos_tt = 1; //tail of tail

    head[0] = GRID_SIZE/2;
    head[1] = GRID_SIZE/2;
    grid[GRID_SIZE/2][GRID_SIZE/2] = 11;
    char move;
    int amp;
    char buffer[100];
    while (fgets(buffer, 100, p_file)){
        sscanf(buffer, "%c %d", &move, &amp);
        process_movement(tails, head, grid, move, amp, &nb_pos_ht, &nb_pos_tt);
    }

    printf("Number of positiom, head of tail: %d\n", nb_pos_ht);
    printf("Number of positiom, tail of tail: %d", nb_pos_tt);

    //Clean memory
    for (int i = 0; i < GRID_SIZE; i++)
    {
        free(grid[i]);
    }
    free(grid);

    for (int i = 0; i < 9; i++)
    {
        free(tails[i]);
    }
    free(tails);

    free(head);

    fclose(p_file);
    
    return 0;
}
