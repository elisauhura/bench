#include <stdio.h>
#include "bench.h"

int main(void) {
    process_init();
    process_name("test");
    process_args(0, NULL);
    process_mode(SEQ);
    process_start_measure();
    process_stop_measure();
    process_append_result("Hello", 5);
    process_append_result("Hello2", 6);
    dump_csv(stdout);
    return 0;
}