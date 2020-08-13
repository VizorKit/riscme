void _put_32(unsigned int, unsigned int);
unsigned int _get_32(unsigned int);
void _dummy(unsigned int);

#define GPIO_CTRL_ADDR 0x10012000
#define GPIO_OUTPUT_EN      (GPIO_CTRL_ADDR+0x08)
#define GPIO_VAL            (GPIO_CTRL_ADDR+0x0C)
#define GPIO_OUT_XOR        (GPIO_CTRL_ADDR+0x40)

#define GPIO_RED_LED        (0x4000000)
#define GPIO_BLUE_LED       (0x2000000)
#define GPIO_GREEN_LED      (0x0800000)
#define GPIO_RGB_PINS       (GPIO_GREEN_LED | GPIO_BLUE_LED | GPIO_RED_LED)

#define MTIME               (0x0200BFF8)
#define MTIME_FREQUENCY     (33)

int vk_start() {
    unsigned int rx;

    _put_32(GPIO_OUTPUT_EN, GPIO_RGB_PINS);
    _put_32(GPIO_VAL,GPIO_RGB_PINS);
    _put_32(GPIO_OUT_XOR,0);
    while(1)
    {
        _put_32(GPIO_VAL,GPIO_RGB_PINS);
        for(rx=0;rx<2000000;rx++) _dummy(rx);
        _put_32(GPIO_VAL,0);
        for(rx=0;rx<2000000;rx++) _dummy(rx);
    }    
    return 0;
} 