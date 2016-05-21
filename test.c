#include <stdio.h>

extern int panicking(int x);

int main(int argc, char ** argv)
{
  printf("main\n");
  printf("panicking? %d\n", panicking(123));
  printf("leave main\n");

  return 0;
}
