/*
This file is to test the output object code.
*/

#include <stdio.h>

int foo();

int main(void)
{
   printf ("Hello from your first program!\n");
   printf ("foo = %i", foo());
   return 0;
}