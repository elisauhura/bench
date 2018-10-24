#ifndef BENCH_H
#define BENCH_H

#include <stdio.h>

enum Bench_mode {
    SEQ = 0,
    OPENMP,
    OMPSS,
    PTHREADS,
    OPTMIZED,
    CUDA
};

void process_name(char * str);
void process_mode(enum Bench_mode mode);
int process_args(int argc, char **argv);

int process_stop_measure(void);
int process_start_measure(void);

int dump_csv(FILE * f); /*Usually STDOUT*/

#endif