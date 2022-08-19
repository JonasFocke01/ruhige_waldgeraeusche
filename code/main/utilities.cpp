#include "utilities.h"

int map_from_to(int x, int in_min, int in_max, int out_min, int out_max) {
    if (x > in_max) {
        return out_max;
    } else {
        return ((x - in_min) * (out_max - out_min)) / (in_max - in_min) + out_min;
    }
}