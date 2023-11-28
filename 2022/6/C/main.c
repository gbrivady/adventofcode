#include <stdio.h>
#include <stdlib.h>

/**
 * Number of != characters in a row to have solution = 1st CL argument
*/
int main(int argc, char const *argv[]){

    if (argc <= 1){
        printf("Provide a challenge parameter!");
        return 1;
    }

    int nb_char = atoi(argv[1]);

    FILE* p_file = fopen("input", "r");
    if (p_file == NULL){
        printf("Input file not found. Check if it exists.\n");
        return 1;
    }


    char* buffer = malloc(4096*sizeof(char));
    fgets(buffer, 4096, p_file);
    int* last_chars = malloc(nb_char*sizeof(int));

    for (int i = 0; i < nb_char; i++)
        last_chars[i] = buffer[i];
    
    int i;
    char cur_char;
    int not_same_in_;
    for (i = nb_char; i < 4096; i++){
        not_same_in_ = 1;
        for (int j = 0; j < nb_char; j++){
            for (int k = 0; k < nb_char; k++){
                if (k != j)
                    not_same_in_ *= (last_chars[j] != last_chars[k]);
            }
        }
        if (not_same_in_)
                break;
        for (int j = 0; j < nb_char-1; j++)
            last_chars[j] = last_chars[j+1];
            
        last_chars[nb_char-1] = buffer[i];
    }
        
    printf("%d", i);

    free(buffer);
    free(last_chars);
    fclose(p_file);
    return 0;
}