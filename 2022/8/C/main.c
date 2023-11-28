#include <stdio.h>
#include <stdlib.h>

int check_left(int** tree_array, int height, int i, int j){
    int view = 0;
    for (int k = j+1; k < 99; k++){
        view++;
        if(height <= tree_array[i][k]){
            return view;
        }
    }
    return view;
}

int check_right(int** tree_array, int height, int i, int j){
    int view = 0;
    for (int k = j-1; k >= 0; k--)
    {
        view++;
        if(height <= tree_array[i][k]){
            return view;
        }
    }
    return view;
}

int check_up(int** tree_array, int height, int i, int j){
    int view = 0;
    for (int k = i-1; k >= 0; k--)
    {
        view++;
        if(height <= tree_array[k][j]){
            return view;
        }
    }
    return view;
}

int check_down(int** tree_array, int height, int i, int j){
    int view = 0;
    for (int k = i+1; k < 99; k++)
    {
        view++;
        if(height <= tree_array[k][j]){
            return view;
        }
    }
    return view;
}

int check_tree(int** tree_array, int i, int j){
    int height = tree_array[i][j];
    int view_score = 1;
    // if(check_down(tree_array, height, i, j)){
    //     (*visible)++;
    //     return;
    // }
    // if(check_up(tree_array, height, i, j)){
    //     (*visible)++;
    //     return;
    // }
    // if(check_left(tree_array, height, i, j)){
    //     (*visible)++;
    //     return;
    // }
    // if(check_right(tree_array, height, i, j)){
    //     (*visible)++;
    //     return;
    // }
    view_score *= check_right(tree_array, height, i,j)*check_left(tree_array, height, i,j)*check_down(tree_array, height, i,j)*check_up(tree_array, height, i,j);
    return view_score;
}

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }

    int** tree_array = calloc(99, sizeof(int*));
    for (int i = 0; i < 99; i++)
    {
        tree_array[i] = calloc(99, sizeof(int));
    }
    char buffer[256];
    for (int i=0; fgets(buffer, 256, p_file); i++){
        for (int j = 0; j < 99; j++)
        {
            tree_array[i][j] = buffer[j]-'0';
        }
    }
    int best_score = 0;
    int cur_score = 0;
    for (int i = 0; i < 99; i++)
    {
        for (int j = 0; j < 99; j++)
        {
            cur_score =check_tree(tree_array, i, j);
            if(cur_score > best_score){
                best_score = cur_score;
            }
        }
        
    }
    printf("%d ", best_score);
    
    return 0;
}
