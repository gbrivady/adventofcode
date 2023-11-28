#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

struct dir{
    struct dir* parent;

    char* name;
    struct dir* leafs;

    int nb_leafs;
    int f_size;
} typedef dir;

void move(char**, dir*);

void do_dir(char** lines, dir* p_cdir){
    // printf("%s \n", p_cdir->name);
    lines++; //goto ls line
    lines++; //goto actual line
    dir* temp_dir;
    while(*lines!= NULL && (*lines)[0] != '$'){
        if(isdigit((*lines)[0])){
            p_cdir->f_size += atoi(*lines);
        } else {
            p_cdir->leafs = realloc(p_cdir->leafs, (p_cdir->nb_leafs+1)*sizeof(dir));
            temp_dir = p_cdir->leafs+p_cdir->nb_leafs;
            
            temp_dir->parent = p_cdir;
            temp_dir->name = malloc(100*sizeof(char));
            sscanf(*lines, "dir %s\n", temp_dir->name);
            temp_dir->leafs = NULL;
            temp_dir->nb_leafs = 0;
            temp_dir->f_size = 0;

            p_cdir->nb_leafs++;
        }
        lines++;
    }
    if(*lines == NULL)
        return;
    move(lines, p_cdir);

}
void move(char** lines, dir* p_cur_dir){
    char* dir_name;
    sscanf(*lines, "$ cd %s\n", dir_name);
    // printf("%s\n", dir_name);
    while(strcmp(dir_name, "..") == 0){
        // printf("going up");
        p_cur_dir = p_cur_dir->parent;
        lines++;
        sscanf(*lines, "$ cd %s\n", dir_name);
    }
    for (int i = 0; i < p_cur_dir->nb_leafs; i++){
        if (strcmp(dir_name, (p_cur_dir->leafs+i)->name) ==0){
            printf(""); // DO NOT REMOVE OR I CRASH
            p_cur_dir = p_cur_dir->leafs+i;
        }
    }
    do_dir(lines, p_cur_dir);
}

int get_size(dir* start, int* tot_size){
    int my_size = start->f_size;
    for (int i = 0; i < start->nb_leafs; i++)
    {
        my_size += get_size((start->leafs+i), tot_size);
    }
    if (my_size < 100000)
    {
        *tot_size += my_size;
    }
    
    return my_size;
}

void find_best_dir(int free_space, int required, dir* p_dir, int* cur_min){
    int x;
    int s;
    for (int i = 0; i < p_dir->nb_leafs; i++){
        find_best_dir(free_space, required, p_dir->leafs+i, cur_min);
        s = get_size(p_dir->leafs+i, &x);
        if (free_space+s > required){
            if(*cur_min > s)
                *cur_min=s;
        }
    }
    
}

int main(int argc, char const *argv[])
{

    FILE* p_file = fopen("input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }

    int len;
    char** lines = NULL;
    char buffer[100];
    for (len = 0; fgets(buffer, 100, p_file); len++)
    {   
        lines = realloc(lines, (len+1)*sizeof(char*));
        lines[len] = malloc(sizeof(char)*100);
        memcpy(lines[len], buffer, 100);
    }
    lines = realloc(lines, (len+1)*sizeof(char*));
    lines[len] = NULL;
    printf("%d \n", len);
    dir* start = malloc(sizeof(dir));

    start->name = "/";
    start->leafs = NULL;
    start->nb_leafs = 0;
    start->parent = NULL;
    start->f_size = 0;

    do_dir(lines, start);
    int* tot = malloc(sizeof(int));
    *tot = 0;
    int free_space = 70000000-get_size(start, tot);
    printf("%d ", *tot);
    printf("%d ", free_space);
    *tot = INT_MAX;
    find_best_dir(free_space, 30000000, start, tot);
    printf("%d", *tot);
    printf("oui");

    return 0;
}
