#include "bench.h"
#include <stdio.h>

int main(int argc, char ** argv) {
    process_start_measure();

    process_name("sample");
    process_mode(SEQ);
    process_args(argc, argv);

    process_stop_measure();

    dump_csv(stdout);
    return 0;
}