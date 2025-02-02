#include "tx.h"
#include <hardware/clocks.h>
#include <pico/time.h>
#include <ctime>
// im lazy lmao

void enter_pair(uint32_t *const fb, uint32_t *bit_streams){
    
    set_sys_clock_48mhz();
    screendisp(fb, bit_streams, "PAIR");


    while (getchar() != 'R'){}
    
    uint64_t cur_time_ns = time_us_64();
    
    printf("{\"myid\": %d,\"scans\":[", code);
    for (int i = 0; i < idx; ++i){
        // printf("%llu\n", times[i]);
        printf("[%hhu, %llu],",scans[i], (cur_time_ns - times[i] ) / 1000000);
    }
    printf("]}");

    set_sys_clock_khz(18000, 1);
    screendisp(fb, bit_streams, "DONE");

}