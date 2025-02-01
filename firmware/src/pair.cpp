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


uint8_t scans[512];

// Quotes

void screendisp(uint32_t *const fb, uint32_t bit_streams[10], const char* str)
{

    for (uint8_t i = 0; i < (string_width(ARRAY_LENGTH(str)) + (FB_WIDTH << 1)); i += 2)
    {
        string_banner_output(fb, (const unsigned char*)str, ARRAY_LENGTH(str), i);
        standard_render_fb(fb, bit_streams);
        if (game_pad.latest_button_press() == ButtonDirection::UP)
            break;
    }
    clear_fb(fb);
    return;
}


#define MYSELF = 0 // 0-255


void render_pair(uint32_t *fb, uint32_t *bit_streams)
{

    const static unsigned char tetris_str[] = "T\205\206\207\206";

    // int i = (string_width(ARRAY_LENGTH(tetris_str)) + (FB_WIDTH << 1)) /2 ;
    int i = 24;
    string_banner_output(fb, tetris_str, ARRAY_LENGTH(tetris_str), i);

    for (int i = 0; i  < 10; ++i){ // prevent overclick
        standard_render_fb(fb, bit_streams); // 10ms

    }

    int arrows[5] = {0,0,0,0,0};
    int count = 0;
    game_pad.reset();
    while (count < ARRAY_LENGTH(arrows)){

        if (game_pad.updated())
        {
            if (game_pad.up.pressed)
            {
                arrows[count] = 0;
            }
            if (game_pad.down.pressed)
            {
                arrows[count] = 1;
            }
            if (game_pad.left.pressed)
            {
                arrows[count] = 2;
            }
            if (game_pad.right.pressed)
            {
                arrows[count] = 3;
            }
            game_pad.reset();
            count += 1;

            sleep_ms(400);
            game_pad.reset();

        }
        standard_render_fb(fb, bit_streams);
    }

    uint8_t out = arrows[0] + (arrows[1]<<2) + (arrows[2]<<4) + (arrows[3]<<6);
    printf("%d id entered!\n", out);

    if (out % 4 == arrows[4]){
        screendisp(fb, bit_streams, "Y ");
        // PUSH
    }
    screendisp(fb, bit_streams, "N" );

}
