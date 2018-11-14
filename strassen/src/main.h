#ifndef KASTORS_MAIN_H
#define KASTORS_MAIN_H

struct user_parameters {
    int check;
    int succeed;
    char* string2display;
    int niter;
/* TITER
    int titer;
*/
    int matrix_size;
    int cutoff_depth;
    int cutoff_size;
/* SMSIZE
    int submatrix_size;
*/
/* BSIZE
    int blocksize;
*/
/*
    int iblocksize;
*/
    
};

extern double run(struct user_parameters* params);

#endif
