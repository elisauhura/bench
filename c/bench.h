#ifndef BENCH_H
#define BENCH_H

#include <stdio.h>

enum Bench_mode {
    SEQ = 0,
    OPENMP,
    PTHREADS,
    OPTMIZED,
    CUDA,
    OPENMP_TASK,
    OMPSS,
    OMPSS2
};

static struct {
    char * name;
    enum Bench_mode mode;
    char * args;
    double begin;
    double end;
    char * out;
    int out_size;
    int out_max;
} bench_data;

void process_init();

void process_name(char * str);
void process_mode(enum Bench_mode mode);
int process_args(int argc, char **argv);

void process_append_result(char * str, int size);

int process_stop_measure(void);
int process_start_measure(void);

#ifdef _OPENMP

int task_init_measure(void);
int task_stop_measure(void);
int task_start_measure(void);

#endif

int dump_csv(FILE * f); /*Usually STDOUT*/

#endif
