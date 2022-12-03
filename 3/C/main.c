#include <stdio.h>
#include <stdlib.h>

int get_index(char c){
    if (c >= 'a')
        return c-'a';
    return c-'A'+26;
}

int get_sack_length(char* line){
    int i = 0;
    for (; line[i] != '\n'; i++);
    return i;
}

int get_sack_prio(char* line){
    int count[52][2] = {0};
    int len = get_sack_length(line);
    int half_len = len/2;

    for (int i = 0; i < len; i++)
        count[get_index(line[i])][i >= half_len] ++;

    for (int i = 0; i < 52; i++)
        if (count[i][0] >0 && count[i][1] >0)
            return i+1;
    
    return 0;
}

int get_group_prio(char** lines){
    int count[52][3] = {0};

    for (int i = 0; i < 3; i++)
        for(int j = 0; lines[i][j] != '\n'; j++)
            count[get_index(lines[i][j])][i] ++;
        
    for (int i = 0; i < 52; i++)
        if (count[i][0] > 0 && count[i][1] > 0 && count[i][2] > 0)
            return i+1;
    return 0;
}

int challenge1(FILE* p_file){
    char buffer[100];
    int sum = 0;
    while (fgets(buffer, 100, p_file)){
        sum += get_sack_prio(buffer);
    }
    return sum;
}

int challenge2(FILE* p_file){
    char buffer1[100];
    char buffer2[100];
    char buffer3[100];
    char* buffers[3] = {buffer1, buffer2, buffer3};
    int sum = 0;
    while (fgets(buffer1, 100, p_file)){
        fgets(buffer2, 100, p_file);
        fgets(buffer3, 100, p_file);
        sum += get_group_prio(buffers);
    }
    return sum;
}

int main(int argc, char const *argv[])
{
    if (argc <= 1){
        printf("Provide a challenge number!");
        return 1;
    }
    int challenge_num = atoi(argv[1])-1;

    FILE* p_file = fopen("input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }

    if (challenge_num){
        printf("%d", challenge2(p_file));
    } else {
        printf("%d", challenge1(p_file));
    }
    
    return 0;
}
