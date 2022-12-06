#include <stdio.h>
#include <stdlib.h>
int main(int argc, char const *argv[]){
    FILE* p_file = fopen("input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }
    char* buffer = malloc(4096*sizeof(char));
    fgets(buffer, 4096, p_file);
    int last_chars[14];
    for (int i = 0; i < 14; i++)
    {
        last_chars[i] = buffer[i];
    }
    
    int i;
    char cur_char;
    int not_same_in_;
    for (i = 14; i < 4096; i++){
        not_same_in_ = 1;
        for (int j = 0; j < 14; j++){
            for (int k = 0; k < 14; k++){
                if (k != j){
                    not_same_in_ *= (last_chars[j] != last_chars[k]);
                }
            }
            }if (not_same_in_){
                break;
            }
        for (int j = 0; j < 13; j++){
            last_chars[j] = last_chars[j+1];
        }
        last_chars[13] = buffer[i];
    }
        
    
    printf("%d", i);
    return 0;
}