#include "quotes.h"

#include "config.h"
#include "letters.h"
#include "render.h"
#include "logo_patterns.h"
#include "letters.h"
#include "pico/rand.h"

#include <stdlib.h>
#include "pico/stdlib.h"
#include "debug.h"
#include <cstring>
#include "input.h"
#include <stdio.h>

extern void screendisp(uint32_t *const fb, uint32_t bit_streams[10], const char* str);

extern uint8_t scans[512];
extern uint64_t times[512];
extern uint8_t idx;
extern int code
;

void enter_pair(uint32_t *const fb, uint32_t *bit_streams);