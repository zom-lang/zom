/*
This file is to test the output object code.
*/

#include <stdio.h>

int foo(int);

int main(void)
{
   printf ("Example, call Zom from C!\n");
   printf ("foo = %i", foo(321));
   return 0;
}