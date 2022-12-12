#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "../../utils/ip_stack.h"

#define min(a, b) (a < b ? a : b)

#define XMAX 41
#define YMAX 171

int g_end[2] = {0};

typedef int point;

void get_coords(point p, int* i, int* j){
    *i = p/1000;
    *j = p%1000;
}
void to_point(point* p, int i, int j){
    *p = i*1000+j;
}

void enqueue_children(point p, bool** visited, int** height, ip_stack* stacked_children){
    int i = 0;
    int j = 0;
    point p_c;
    get_coords(p, &i, &j);
    int h = height[i][j];
    for (int di = -1; di <= 1; di++){
        if(i+di >= 0 && i+di < XMAX){
            if (!visited[i+di][j] && height[i+di][j] <= h+1){
                visited[i+di][j] = true;
                to_point(&p_c, i+di, j);
                push(stacked_children, p_c);
            }
        }
    }
    for (int dj = -1; dj <= 1; dj++){
        if(j+dj >= 0 && j+dj < YMAX){
            if (!visited[i][j+dj] && height[i][j+dj] <= h+1){
                visited[i][j+dj] = true;
                to_point(&p_c, i, j+dj);
                push(stacked_children, p_c);
            }
            
        }
    }
}

int bfs(point start, bool** visited, int** height){
    int i = 0;
    ip_stack s1 = get_empty(64);
    i++;
    enqueue_children(start, visited, height, &s1);
    while (s1.size != 0){
        ip_stack s2 = get_empty(64);
        i++;
        while (s1.size != 0){
            enqueue_children(pop(&s1), visited, height, &s2);
        }
        if(visited[g_end[0]][g_end[1]])
            return i;
        free_stack(&s1);
        s1.content = s2.content;
        s1.size = s2.size;
        s1._max_size = s2._max_size;
    }
    free_stack(&s1);
    return INT_MAX;
}

void reset_visited(bool** visited){
    for (int i = 0; i < XMAX; i++){
        for (int j = 0; j < YMAX; j++){
            visited[i][j] = false;
        }
    }
}

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("../input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }

    char buffer[200];
    int** height = calloc(XMAX, sizeof(int*));
    for (int i = 0; i < XMAX; i++){
        height[i] = calloc(YMAX, sizeof(int));
    }
    bool** visited = calloc(XMAX, sizeof(bool*));
    for (int i = 0; i < XMAX; i++){
        visited[i] = calloc(YMAX, sizeof(bool));
    }

    int start[2] = {-1};
    char cur_char;
    int i = 0;
    int j = 0;
    while (fgets(buffer, 200, p_file)){
        for (int j = 0; buffer[j] != '\n'; j++){
            visited[i][j] = 0;
            cur_char = buffer[j];
            if (cur_char == 'S'){
                start[0] = i;
                start[1] = j;
                height[i][j] = 0;
                continue;
            } else if(cur_char == 'E'){
                g_end[0] = i;
                g_end[1] = j;
                height[i][j] = 25;
                continue;
            }
            height[i][j] = cur_char - 'a';
        }
        i++;
    }
    point start_p;
    to_point(&start_p, start[0], start[1]);
    printf("Part 1 : %d\n", bfs(start_p, visited, height));

    int cur_min = INT_MAX;
    int dist = 0;
    for (int i = 0; i < XMAX; i++){
        for (int j = 0; j < YMAX; j++)
        {
            if (height[i][j] == 0){
                reset_visited(visited);
                to_point(&start_p, i, j);
                dist = bfs(start_p, visited, height);
                cur_min = min(dist, cur_min);
            }
        }
        
    }
    
    printf("Part 2 : %d", cur_min);
    return 0;
}
