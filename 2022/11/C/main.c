#include <stdio.h>
#include <stdlib.h>
#include <string.h>
// #include "../../utils/ip_stack.h"
// #include "../../utils/ip_stack.c"

#define NB_MONKEY 8

struct monkey
{
    int** items;
    int nb_items;
    int mod;
    int t;
    int f;
    int act;
} typedef monkey;

void do_op(int monkey, int* p_n){
    switch (monkey)
    {
    case 0:
        *p_n *= 13;
        break;
    case 1:
        *p_n += 3;
        break;
    case 2:
        *p_n += 6;
        break;
    case 3:
        *p_n += 2;
        break;
    case 4:
        *p_n *= *p_n;
        break;
    case 5:
        *p_n += 4;
        break;
    case 6:
        *p_n *= 7;
        break;
    case 7:
        *p_n += 7;
        break;
    default:
        break;
    }
    // *p_n /= 3;
}

void push_new(monkey** m_list, int m_n, int value){
    monkey* p_m = m_list[m_n];
    p_m->items[p_m->nb_items] = calloc(8, sizeof(int));
    for (int i = 0; i < NB_MONKEY; i++){
        p_m->items[p_m->nb_items][i] = value;
    }
    p_m->nb_items++;
}

void move(monkey** m_list, int m_src, int m_des){
    monkey* p_m_src = m_list[m_src];
    monkey* p_m_des = m_list[m_des];
    p_m_des->items[p_m_des->nb_items] = p_m_src->items[p_m_src->nb_items-1];
    p_m_des->nb_items++;
    p_m_src->nb_items--;
}

void do_monkey(monkey** m_list, int m_n){
    monkey* p_m = m_list[m_n];
    while (p_m->nb_items){
        for (int i = 0; i < NB_MONKEY; i++){
            do_op(m_n, p_m->items[p_m->nb_items-1]+i);
            p_m->items[p_m->nb_items-1][i] %= (m_list[i]->mod);
        }
        if(p_m->items[p_m->nb_items-1][m_n]){
            move(m_list, m_n, p_m->f);
        }else{
            move(m_list, m_n, p_m->t);
        }
        p_m->act++;
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

    char buffer[100];
    int value0, value1, value2, value3, value4, value5, value6, value7;
    int* values[8] = {&value0, &value1, &value2, &value3, &value4, &value5, &value6, &value7};
    int nb_vals = 0;
    char* ptr;

    monkey* m_list[NB_MONKEY];
    for (int i = 0; i < NB_MONKEY; i++)
    {
        m_list[i] = malloc(sizeof(monkey));
        m_list[i]->items=calloc(64, sizeof(int*));
        m_list[i]->nb_items=0;
        m_list[i]->act = 0;
    }

    for (int i = 0; i < NB_MONKEY; i++)
    {   
        fgets(buffer, 100, p_file);
        fgets(buffer, 100, p_file);
        //read nums
        nb_vals = sscanf(buffer, "  Starting items: %d, %d, %d, %d, %d, %d, %d, %d", &value0, &value1, &value2, &value3, &value4, &value5, &value6, &value7);
        for (int j = 0; j < nb_vals; j++){
            push_new(m_list, i, *(values[j]));
        }
        //end
        fgets(buffer, 100, p_file); //operation

        fgets(buffer, 100, p_file); //Test
        sscanf(buffer, "  Test: divisible by %d", &(m_list[i]->mod));
        
        fgets(buffer, 100, p_file); //if true
        sscanf(buffer, "    If true: throw to monkey %d", &(m_list[i]->t));

        fgets(buffer, 100, p_file); //if false
        sscanf(buffer, "    If false: throw to monkey %d", &(m_list[i]->f));

        fgets(buffer, 100, p_file);
    }
    for (int i = 0; i < 10000; i++)
    {
        for (int j = 0; j < NB_MONKEY; j++)
        {
            do_monkey(m_list, j);
        }
        
    }
    long long act_top_mon = m_list[0]->act;
    long long act_sec_mon = 0;
    for (int i = 1; i < NB_MONKEY; i++)
    {
        if (m_list[i]->act > act_top_mon)
        {
            act_sec_mon = act_top_mon;
            act_top_mon = m_list[i]->act;
        } else if(m_list[i]->act > act_sec_mon){
            act_sec_mon = m_list[i]->act;
        }
    }
    
    printf("%lld", act_sec_mon*act_top_mon);
    return 0;
}
