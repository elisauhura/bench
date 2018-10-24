/*double rtclock()
{
    struct timezone Tzp;
    struct timeval Tp;
    int stat;
    stat = gettimeofday (&Tp, &Tzp);
    if (stat != 0) printf("Error return from gettimeofday: %d",stat);
    return(Tp.tv_sec + Tp.tv_usec*1.0e-6);
}*/
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

#include "bench.h"

static struct {
    char * name;
    enum Bench_mode mode;
    char * args;
    double begin;
    double end;
} bench_data;

static int str_size(char * str) {
    int i;
    for(i = 0; str[i] != '\0'; i++) {
        switch(str[i]) {
            case '\b':
            case '\\':
            case '\f':
            case '\n':
            case '\r':
            case '\t':
            case '\"':
                i++;
        }
    };
    return i+1;
};

char * mode[] = { "SEQ", "OPENMP", "OMPSS", "PTHREADS", "OPTMIZED", "CUDA" };

static double rtclock()
{
    struct timeval t;
    gettimeofday(&t, NULL);
    return t.tv_sec + t.tv_usec*1e-6;
}

void process_name(char * str) {
    bench_data.name = str;
}

void process_mode(enum Bench_mode mode) {
    bench_data.mode = mode;
}

int process_args(int argc, char **argv) {
    int i = 0;
    for(int s = 0; s < argc; s++)
        i += str_size(argv[s]);
    char * q = malloc(i);
    if(q == NULL) return 0;
    i = 0;
    for(int s = 0; s < argc; s++) {
        for(int j = 0; argv[s][j] != '\0'; j++) {
            switch(argv[s][j]) {
            case '\b':
                q[i++] = '\\';
                q[i++] = 'b';
                break;
            case '\\':
                q[i++] = '\\';
                q[i++] = '\\';
                break;
            case '\f':
                q[i++] = '\\';
                q[i++] = 'f';
                break;
            case '\n':
                q[i++] = '\\';
                q[i++] = 'n';
                break;
            case '\r':
                q[i++] = '\\';
                q[i++] = 'r';
                break;
            case '\t':
                q[i++] = '\\';
                q[i++] = 't';
                break;
            case '\"':
                q[i++] = '\\';
                q[i++] = '"';
                break;
            default:
                q[i++] = argv[s][j];
            }
        }
        q[i++] = ' ';
    }
    q[--i] = '\0';
    bench_data.args = q;
    return 1;
}

int process_stop_measure(void) {
    bench_data.begin = rtclock();
}
int process_start_measure(void) {
    bench_data.end = rtclock();
}

int dump_csv(FILE * f) {
    fprintf(f, "{\"bench\" : \"%s\",\"mode\" : \"%s\",\"args\" : \"%s\",\"time\" : %lf}\n", bench_data.name, mode[bench_data.mode], bench_data.args, bench_data.end - bench_data.begin);
}