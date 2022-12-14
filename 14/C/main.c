#include <stdlib.h>
#include <stdio.h>

#define min(a, b) (a < b ? a : b)
#define max(a, b) (a > b ? a : b)

#define CAVE_DEPTH 173
#define CAVE_WIDTH 1000
#define MIN_X 0

void find_space(FILE* p_file){
    int tempx, tempy, minx, miny, maxx, maxy;
    minx = INT_MAX;
    miny = INT_MAX;
    maxx = INT_MIN;
    maxy = INT_MIN;

    char buffer[500];

    char** startptr = malloc(sizeof(char*));

    while (fgets(buffer, 500, p_file)){
        *startptr = buffer;
        tempx = strtol(*startptr, startptr, 10);
        (*startptr)++; // ','
        tempy = strtol(*startptr, startptr, 10);
        minx = min(minx, tempx);
        maxx = max(maxx, tempx);
        miny = min(miny, tempy);
        maxy = max(maxy, tempy);
        while (**startptr != '\n'){
            // space, -, >, space
            (*startptr) += 4;
            tempx = strtol(*startptr, startptr, 10);
            (*startptr)++; // ','
            tempy = strtol(*startptr, startptr, 10);
            minx = min(minx, tempx);
            maxx = max(maxx, tempx);
            miny = min(miny, tempy);
            maxy = max(maxy, tempy);
        }
    }
    printf("%d %d; %d %d\n", minx, maxx, miny, maxy);
    free(startptr);
    fclose(p_file);
}

void print_cave(short** cave);

void draw_line(int x1, int x2, int y1, int y2, short** cave_array){
    if (x1 == x2){
        for (int i = min(y1, y2); i <= max(y2, y1); i++){
            cave_array[i][x1-MIN_X+1] = 1;
        } 
    } else {
        for (int i = min(x1, x2); i <= max(x1, x2); i++){
            cave_array[y1][i-MIN_X+1] = 1;
        } 
    }
}

void build_cave(FILE* p_file, short** cave_array){
    char buffer[500];
    int x1, y1, x2, y2;
    char** startptr = malloc(sizeof(char*));
    while (fgets(buffer, 500, p_file)){

        *startptr = buffer;
        x1 = strtol(*startptr, startptr, 10);
        (*startptr)++; // ','
        y1 = strtol(*startptr, startptr, 10);
        
        while (**startptr != '\n'){
            // space, -, >, space
            (*startptr) += 4;
            x2 = strtol(*startptr, startptr, 10);
            (*startptr)++; // ','
            y2 = strtol(*startptr, startptr, 10);
            draw_line(x1, x2, y1, y2, cave_array);
            x1 = x2;
            y1 = y2;
        }
    }
    draw_line(0, CAVE_WIDTH-1, 170, 170, cave_array);
    
    free(startptr);
}

void print_cave(short** cave){
    for (int y = 0; y < CAVE_DEPTH; y++)
    {
        for (int x = 0; x < CAVE_WIDTH; x++)
        {
            putchar(cave[y][x] == 0 ? '.' : (cave[y][x] == 1 ? '#' : 'o'));
        }
        putchar('\n'); 
    }
    
}
enum sable_move {DOWN, LEFT, RIGHT, STAY};

enum sable_move fall_sand(int x, int y, short** cave){
    //check if free space under
    if (!cave[y+1][x]){
        return DOWN; //it can go down
    } else if(!cave[y+1][x-1]){ //if there is space on the down-left
        return LEFT;
    } else if(!cave[y+1][x+1]){ //down right
        return RIGHT;
    }
    return STAY;
}

int main(int argc, char const *argv[]){
    
    FILE* p_file = fopen("../input", "r");
    if (p_file == NULL){
        printf("File not found.");
        exit(1);
    }
    // find_space(p_file); x in [483, 534] and y in [13, 168]; sand start from (500, 0)
    // access will be in [y][x] as per definition of the cave
    short** cave = calloc(CAVE_DEPTH, sizeof(short*));

    for (int y = 0; y < CAVE_DEPTH; y++){
        cave[y] = calloc(CAVE_WIDTH, sizeof(short));
        for (int x = 0; x < CAVE_WIDTH; x++){
            cave[y][x] = 0;
        }
    }
    build_cave(p_file, cave);

    int x;
    int y;
    int sand = 0;
    enum sable_move move;
    int part1 = 0;
    do{
        y = 0;
        x = 500-MIN_X+1;
        do{
            move = fall_sand(x, y, cave);
            switch (move){
            case STAY:
                break;
            case LEFT:
                y++;
                x--;
                break;
            case RIGHT:
                y++;
                x++;
                break;
            case DOWN:
                y++;
                break;
            default:
                break;
            }
            if (!part1 && y > 168){
                part1 = 1;
                printf("Part 1: %d\n", sand);
            }
            
        } while (move != STAY);
        if (move == STAY){
            cave[y][x] = 2;
            sand++;
        }
    } while (cave[0][500-MIN_X+1] != 2);
    cave[y][x] = 2;

    
    // print_cave(cave);

    printf("Part 2 : %d", sand);

    fclose(p_file);
    for (int y = 0; y < CAVE_DEPTH; y++){
        free(cave[y]);
    }
    free(cave);
    return 0;
}
