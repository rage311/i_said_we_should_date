#include <stdio.h>
#include <time.h>

int main(int argc, char *argv[]) {
  time_t t_bag = time(NULL);
  printf("%ld\n", t_bag);
  struct tm *tm_local = localtime(&t_bag);
  printf("Current local time and date: %s", asctime(tm_local));
  return 0;
}
