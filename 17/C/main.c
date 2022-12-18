#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define NB_ROCKS 2022
#define NB_ROCKS_P2 1000000000000u

#define MAX_CYCLE_LEN 10000

#define MAX_HEIGHT 4*MAX_CYCLE_LEN+3
#define CHAMBER_WIDTH 7

#define max(a, b) (a > b ? a : b)

#define g_piece_minus {{0,0,0,0},{0,0,0,0},{0,0,0,0},{1,1,1,1}}

#define g_piece_plus  {{0,0,0,0},{0,1,0,0},{1,1,1,0},{0,1,0,0}}

#define g_piece_revl  {{0,0,0,0},{0,0,1,0},{0,0,1,0},{1,1,1,0}}

#define g_piece_vert  {{1,0,0,0},{1,0,0,0},{1,0,0,0},{1,0,0,0}}

#define g_piece_squa  {{0,0,0,0},{0,0,0,0},{1,1,0,0},{1,1,0,0}}

enum pieces {MINUS = 0, PLUS = 1, REVERSED_L = 2, VERTICAL_BAR = 3, SQUARE = 4};

enum move {DOWN, LEFT, RIGHT};

int g_piece_arr[5][4][4] = {g_piece_minus, g_piece_plus, g_piece_revl, g_piece_vert, g_piece_squa};

void add_piece(int piece, int(* playground)[MAX_HEIGHT][CHAMBER_WIDTH], int piece_i, int piece_j){
    for (int i = 0; i < 4 ; i++){
        for (int j = 0; j < 4; j++){
            (*playground)[piece_i+i][piece_j+j] += g_piece_arr[piece][3-i][j];
        }
    }
}

void print_playground(int(* playground)[MAX_HEIGHT][CHAMBER_WIDTH], int max_height){
    for (int i = max_height; i >= 0; i--){
        putchar('|');
        for (int j = 0; j < CHAMBER_WIDTH; j++){
            putchar((*playground)[i][j]? '#' : '.');
        }
        putchar('|');
        putchar('\n');
    }
    printf("+-------+\n");
    
}

bool is_move_valid(int piece_i, int piece_j, int piece, enum move input, int(* playground)[MAX_HEIGHT][CHAMBER_WIDTH]){
    int di = 0;
    int dj = 0;
    switch (input){
    case DOWN:
        di = -1;
        break;
    case LEFT:
        dj = -1;
        break;
    case RIGHT:
        dj = +1;
        break;
    default:
        break;
    }
    bool temp_cond;
    for (int i = 0; i < 4; i++){
        for (int j = 0; j < 4; j++){
            temp_cond = true;
            // //check that the piece has a part there
            // temp_cond = g_piece_arr[3-i][j]

            //check that it does not go oob
            temp_cond *= (piece_j+dj+j < CHAMBER_WIDTH) && (piece_j+dj+j >= 0); // left-right bound
            temp_cond *= (piece_i+di+i >= 0); // bottom of the playground
            //check that there is no piece there
            temp_cond *= !((*playground)[piece_i+di+i][piece_j+dj+j]);

            //if something is wrong _and_ there is a part of the piece:
            if(!temp_cond && g_piece_arr[piece][3-i][j]){
                return false;
            }
        } 
    }
    return true;
}

int signature(int piece, int piece_j, int move_nb){
    return piece+piece_j*10+move_nb*100;
}

int main(int argc, char const *argv[])
{
    FILE* p_file = fopen("../input", "r");
    if (p_file == NULL){
        printf("File not found.");
        exit(1);
    }

    int height_at_rock[MAX_CYCLE_LEN];
    int signature_arr[MAX_CYCLE_LEN];
    int playground[MAX_HEIGHT][CHAMBER_WIDTH] = {0};
    int max_height = -1;
    int piece_i;
    int piece_j;
    char cur_move = 'a';

    int move_nb = 0;

    int piece;
    bool go_down;
    for (unsigned long long piece_nb = 0; piece_nb < MAX_CYCLE_LEN; piece_nb++){
        piece = piece_nb%5;
        piece_i = max_height+4;
        // printf("%d \n", max_height);
        piece_j = 2;
        do
        {
            cur_move = getc(p_file);
            if(cur_move == '\n'){
                rewind(p_file);
                move_nb = 0;
                cur_move = getc(p_file);
            }
            move_nb++;
            if (cur_move == '<'){
                if(is_move_valid(piece_i, piece_j, piece, LEFT, &playground))
                    piece_j--;
            } else if(cur_move == '>'){
                if(is_move_valid(piece_i, piece_j, piece, RIGHT, &playground))
                    piece_j++;
            }
            
            go_down = is_move_valid(piece_i, piece_j, piece, DOWN, &playground);
            if(go_down)
                piece_i--;
        } while (go_down);

        signature_arr[piece_nb] = signature(piece, piece_j, move_nb);

        add_piece(piece, &playground, piece_i, piece_j);
        if (piece_nb < 0)
        {
            print_playground(&playground, 10);
        }
        
        switch (piece)
        {
        case MINUS:
            max_height = max(max_height, piece_i);
            break;
        case PLUS:
            max_height = max(max_height, piece_i+2);
            break;
        case REVERSED_L:
            max_height = max(max_height, piece_i+2);
            break;
        case VERTICAL_BAR:
            max_height = max(max_height, piece_i+3);
            break;
        case SQUARE:
            max_height = max(max_height, piece_i+1);
            break;
        default:
            break;
        }
        height_at_rock[piece_nb] = max_height;
    }
    

    print_playground(&playground, 21);
    printf("Max_heigth part 1 %d\n", height_at_rock[NB_ROCKS-1]+1);
    
    // TODO : actual code to compute everything
    // int sig;
    // for (int i = 0; i < MAX_CYCLE_LEN; i++){
    //     sig = signature_arr[i];
    //     printf("Found signature %d at rocks_nb: %d ", sig, i);
    //     for (int j = i+1; j < MAX_CYCLE_LEN; j++)
    //     {
    //         sig == signature_arr[j] ? printf("%d %d ", j, height_at_rock[j]) : putchar('\0');
    //     }
        
    //     putchar('\n');
    // }
    // CYCLE START : 174
    // CYCLE LEN : 1 740
    // CYCLE H_INCREASE : 2 724
    // VALUE AT NB_ROCKS_P2: 2 724*[NB_ROCKS-P2-174]/1740 + Height[(NB_ROCKS_P2-174)%1740]

    printf("Max_height part2 : %lld\n", 2724*((NB_ROCKS_P2-174)/1740)+height_at_rock[(NB_ROCKS_P2-174)%1740]+height_at_rock[174]);
    return 0;
}
