#include <stdio.h>
#include "hello.h"

int main(int argc, char const* argv[])
{
    if (argc != 2)
    {
        say_hello(NULL);
    }
    else
    {
        say_hello(argv[1]);
    }

    return 0;
}
