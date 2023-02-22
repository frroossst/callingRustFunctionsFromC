#include <stdint.h>
#include <stdio.h>
#include "rust.h"

int main()
    {
    printf("Hello World! \n");

    printf("%d\n", give_an_u8());
    printf("%d\n", pass_and_get_an_u8(100));

    return 0;
    }
