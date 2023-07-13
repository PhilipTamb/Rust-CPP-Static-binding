#include <iostream>
#include <stdio.h>
//#include "myheader.h"

extern "C" int covert_and_sum (int a, int b){
    int y = static_cast<int>(b);
    return a + y;
}

int main(){

    int x = covert_and_sum(5, 10);
    printf("Number %d\n", x);

}

