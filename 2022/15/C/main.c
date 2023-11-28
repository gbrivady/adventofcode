#include <stdlib.h>
#include <stdio.h>

#define TARGET_Y 2000000

struct sensor{
    long sx;
    long sy;
    long bx;
    long by;
} typedef sensor;

long dist_p(int a, int b, int c, int d){
    return abs(a-c)+abs(b-d);
}

long dist(sensor s){
    return abs(s.sx-s.bx)+abs(s.sy-s.by);
}

int main(int argc, char const *argv[]){

    FILE* p_file = fopen("../input", "r");
    if (p_file == NULL){
        printf("File not found.");
        exit(1);
    }

    sensor s_arr[100];
    char buffer[100];
    int nb_sens = 0;
    short* big_arr = calloc(7000000, sizeof(short));
    for (int i = 0; i <     7000000; i++){
        big_arr[i] = 0;
    }
    
    printf("big array allocated \n");
    while (fgets(buffer, 100, p_file)){
        sscanf(buffer, "Sensor at x=%ld, y=%ld: closest beacon is at x=%ld, y=%ld", 
        &s_arr[nb_sens].sx,
        &s_arr[nb_sens].sy,
        &s_arr[nb_sens].bx,
        &s_arr[nb_sens].by);
        if (s_arr[nb_sens].by == TARGET_Y){
            big_arr[s_arr[nb_sens].bx+1000000] = 2;
        }
        
        nb_sens++;
    }

    // int intervals[100][2];
    int nb_int = 0;

    long d;
    int y_shift;
    long y_dist = 0;
    long ta;
    long new_mn_x;
    long new_mx_x;
    long nb_truc = 0;

    for (int s = 0; s < nb_sens; s++){
        d = dist(s_arr[s]);
        //can we reach y = 2 000 000?
        y_dist = abs(s_arr[s].sy-TARGET_Y);
        if (y_dist <= d){
            new_mn_x = s_arr[s].sx-(d-y_dist);
            new_mx_x = s_arr[s].sx+(d-y_dist);
            for (int i = new_mn_x; i <= new_mx_x; i++){
                if(big_arr[i+1000000] == 0){
                    big_arr[i+1000000] = 1;
                    nb_truc += 1;
                }
            }
            
        }
            
    }
    printf("Part 1: %d ", nb_truc);

    // Look for the beacon
    // There must be only one solution, as per the question, thus
    // it needs be at en edge, and circled by beacons, or there would be 2
    // solutions. So we can only test on the edges of every beacons - still a large
    // space, but much smaller
    sensor s;
    int y;
    int d_;
    int dm, dp;
    printf("Part 2: ");
    long x_cord = -1;
    long y_cord = 0;
    int s_;
    for (int s = 0; s < nb_sens; s++){
        //the set of points at the edge:
        // found a start point, then move in diagonal upto
        d = dist(s_arr[s]);
        for (int x = -(d+1); (x <= (d+1) && x_cord==-1); x++){
            y = d+1 - abs(x);
            if((0 <= s_arr[s].sx+x) && (s_arr[s].sx+x <= 2*TARGET_Y) && (0 <= s_arr[s].sy+y) && (s_arr[s].sy+y <= 2*TARGET_Y)){
                for (s_ = 0; s_ < nb_sens; s_++){
                    d_ = dist(s_arr[s_]);
                    dp = dist_p(s_arr[s_].sx, s_arr[s_].sy, s_arr[s].sx+x, s_arr[s].sy+y);
                    if (dp < d_+1){ //not on the edge or farther away
                        s_ = 0;
                        break;
                    }
                }
                if (s_ == nb_sens){ // no break
                    x_cord = (s_arr[s].sx+x);
                    y_cord = (s_arr[s].sy+y);
                }
            }

            if((0 <= s_arr[s].sx+x) && (s_arr[s].sx+x <= 2*TARGET_Y) && (0 <= s_arr[s].sy-y) && (s_arr[s].sy-y <= 2*TARGET_Y)){
                for (s_ = 0; s_ < nb_sens; s_++){
                    d_ = dist(s_arr[s_]);
                    dp = dist_p(s_arr[s_].sx, s_arr[s_].sy, s_arr[s].sx+x, s_arr[s].sy-y);
                    if (dp < d_+1){ //not on the edge or farther away
                        s_ = 0;
                        break;
                    }
                }
                if (s_ == nb_sens){ // no break
                    x_cord = (s_arr[s].sx+x);
                    y_cord = (s_arr[s].sy-y);
                }
            }
        }
    }
    printf("(%ld, %ld), %lld", x_cord, y_cord, x_cord*2LL*TARGET_Y+y_cord); 
    free(big_arr);
    return 0;
}