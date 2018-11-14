#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <float.h>
#include <math.h>

#include "../c/bench.h"

#define MSIZE
#define CUTOFF_SIZE
#define CUTOFF_DEPTH

#ifdef _OPENMP
#  include <omp.h>
#endif

#include "main.h"

#define min(a, b) ((a<b)?a:b)
#define max(a, b) ((a>b)?a:b)

void parse(int argc, char* argv[], struct user_parameters* params)
{
    int i;
    for(i=1; i<argc; i++) {
        if(!strcmp(argv[i], "-c"))
            params->check = 1;
        else if(!strcmp(argv[i], "--help") || !strcmp(argv[i], "-h")) {
            printf("----------------------------------------------\n");
            printf("-                KaStORS                     -\n");
            printf("-   Kaapi Starpu OpenMP Runtime task Suite   -\n");
            printf("----------------------------------------------\n");
            printf("-h, --help : Show help information\n");
            printf("-c : Ask to check result\n");
            printf("-i : Number of iterations\n");
#ifdef TITER
            printf("-r : Number ot timestep iteration\n");
#endif
#ifdef MSIZE
            printf("-n : Matrix size\n");
#endif
#ifdef SMSIZE
            printf("-m : SubMatrix size\n");
#endif
#ifdef BSIZE
            printf("-b : Block size\n");
#endif
#ifdef IBSIZE
            printf("-ib : Internal Block size\n");
#endif
#ifdef CUTOFF_SIZE
            printf("-s : Cutoff (Size of the matrix)\n");
#endif
#ifdef CUTOFF_DEPTH
            printf("-d : Cutoff (depth)\n");
#endif
            exit(EXIT_SUCCESS);
        } else if(!strcmp(argv[i], "-i")) {
            if (++i < argc)
                params->niter = atoi(argv[i]);
            else {
                fprintf(stderr, "-i requires a number\n");
                exit(EXIT_FAILURE);
            }
#ifdef TITER
        } else if(!strcmp(argv[i], "-r")) {
            if (++i < argc)
                params->titer = atoi(argv[i]);
            else {
                fprintf(stderr, "-r requires a number\n");
                exit(EXIT_FAILURE);
            }
#endif
#ifdef MSIZE
        } else if(!strcmp(argv[i], "-n")) {
            if (++i < argc)
                params->matrix_size = atoi(argv[i]);
            else {
                fprintf(stderr, "-n requires a number\n");
                exit(EXIT_FAILURE);
            }
#endif
#ifdef SMSIZE
        } else if(!strcmp(argv[i], "-m")) {
            if (++i < argc)
                params->submatrix_size = atoi(argv[i]);
            else {
                fprintf(stderr, "-m requires a number\n");
                exit(EXIT_FAILURE);
            }
#endif
#ifdef BSIZE
        } else if(!strcmp(argv[i], "-b")) {
            if (++i < argc)
                params->blocksize = atoi(argv[i]);
            else {
                fprintf(stderr, "-b requires a number\n");
                exit(EXIT_FAILURE);
            }
#endif
#ifdef IBSIZE
        } else if(!strcmp(argv[i], "-ib")) {
            if (++i < argc)
                params->iblocksize = atoi(argv[i]);
            else {
                fprintf(stderr, "-ib requires a number\n");
                exit(EXIT_FAILURE);
            }
#endif
#ifdef CUTOFF_SIZE
        } else if(!strcmp(argv[i], "-s")) {
            if (++i < argc)
                params->cutoff_size = atoi(argv[i]);
            else {
                fprintf(stderr, "-s requires a number\n");
                exit(EXIT_FAILURE);
            }
#endif
#ifdef CUTOFF_DEPTH
        } else if(!strcmp(argv[i], "-d")) {
            if (++i < argc)
                params->cutoff_depth = atoi(argv[i]);
            else {
                fprintf(stderr, "-d requires a number\n");
                exit(EXIT_FAILURE);
            }
#endif
        } else
            fprintf(stderr, "Unknown parameter : %s\n", argv[i]);
    }
}

int comp (const void * elem1, const void * elem2) 
{
    double f = *((double*)elem1);
    double s = *((double*)elem2);
    if (f > s) return  1;
    if (f < s) return -1;
    return 0;
}

int main(int argc, char* argv[])
{
    int num_threads = 1;
    struct user_parameters params;
    memset(&params, 0, sizeof(params));

    process_name("c-ray-mt");
    process_mode(SEQ);
    process_args(argc, argv);

    /* default value */
    params.niter = 1;

    parse(argc, argv, &params);

// get Number of thread if OpenMP is activated
#ifdef _OPENMP
    #pragma omp parallel
    #pragma omp master
    num_threads = omp_get_num_threads();
    process_mode(OPENMP);
    task_init_measure();
#endif
    //warmup
    run(&params);

    process_start_measure();
        run(&params);
    process_stop_measure();

    dump_csv(stdout);
    return 0;
}
