#ifndef ELF_H
#define ELF_H
#include <stdio.h>

struct elf
{
    int elf_number;
    int cal;
} typedef elf;

elf make_elf(FILE* cur_file);

/// @brief Compare the number of calories between Galadriel and Legolas
/// @param galadriel 
/// @param legolas 
/// @return 0 if Galadriel has strictly more Calories than Legolas, 1 else.
int compare_elves(elf galadriel, elf legolas);

void rerank_elves(elf* p_first, elf* p_second, elf* p_third, elf* p_newbie);

#endif