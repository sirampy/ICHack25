#include "tx.h"
#include <hardware/clocks.h>
#include <pico/time.h>
// im lazy lmao

void enter_pair(uint32_t *const fb, uint32_t *bit_streams){
    
    set_sys_clock_48mhz();
    screendisp(fb, bit_streams, "PAIR");


    while (getchar() != 'R'){}
    

    printf("[");
    for (int i = 0; i < idx; ++i){
        printf("[%hhu, %d],",scans[i], times[i]);
    }
    printf("]");

    set_sys_clock_khz(18000, 1);
    screendisp(fb, bit_streams, "DONE");


}