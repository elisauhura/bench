CC=gcc
FLAGS_OMP=-fopenmp -lm
FLAGS_OPT=-O3 -ffast-math -D_GNU_SOURCE -lm -lpthread
FLAGS_PTH=-O3 -ffast-math -lm -lpthread
FLAGS_SEQ=-O3 -ffast-math -lm

build:
	$(CC) src/omp/c-ray-mt.c $(FLAGS_OMP) -o bin/c-ray-omp
	$(CC) src/optimized/c-ray-mt.c $(FLAGS_OPT) -o bin/c-ray-opt
	$(CC) src/pthread/c-ray-mt.c $(FLAGS_PTH) -o bin/c-ray-pth
	$(CC) src/seq/c-ray-mt.c $(FLAGS_SEQ) -o bin/c-ray-seq

run:
	./bin/c-ray-omp -s 4000x4000 -i input/scene -o output/omp_scene.ppm
	./bin/c-ray-seq -s 4000x4000 -i input/scene -o output/seq_scene.ppm
	./bin/c-ray-opt -s 4000x4000 -i input/scene -o output/opt_scene.ppm
	./bin/c-ray-pth -s 4000x4000 -i input/scene -o output/pth_scene.ppm
	./bin/c-ray-omp -s 4000x4000 -i input/sphfract -o output/omp_sphfract.ppm
	./bin/c-ray-seq -s 4000x4000 -i input/sphfract -o output/seq_sphfract.ppm
	./bin/c-ray-opt -s 4000x4000 -i input/sphfract -o output/opt_sphfract.ppm
	./bin/c-ray-pth -s 4000x4000 -i input/sphfract -o output/pth_sphfract.ppm
