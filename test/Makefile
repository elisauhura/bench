CC=gcc
FLAGS_OMP=-fopenmp -lm
FLAGS_OPT=-O3 -ffast-math -D_GNU_SOURCE -lm -lpthread
FLAGS_PTH=-O3 -ffast-math -lm -lpthread
FLAGS_SEQ=-O3 -ffast-math -lm
FLAGS_API_REQ=-lssl -lcrypto
G=

bench:
	$(CC) $(G) -c c/bench.c -o bench.o
	$(CC) $(G) -c c/bench.c -o bench_omp.o -fopenmp

build: bench
	$(CC) $(G) src/omp/c-ray-mt.c bench_omp.o $(FLAGS_OMP) -o bin/c-ray-omp $(FLAGS_API_REQ)
	# $(CC) $(G) src/optimized/c-ray-mt.c bench.o $(FLAGS_OPT) -o bin/c-ray-opt
	$(CC) $(G) src/pthread/c-ray-mt.c bench.o $(FLAGS_PTH) -o bin/c-ray-pth $(FLAGS_API_REQ)
	$(CC) $(G) src/seq/c-ray-mt.c bench.o $(FLAGS_SEQ) -o bin/c-ray-seq $(FLAGS_API_REQ)

run:
	echo "{ \"out\" : ["
	./bin/c-ray-omp -s 4000x4000 -i input/scene -o output/omp_scene.ppm
	echo ','
	./bin/c-ray-seq -s 4000x4000 -i input/scene -o output/seq_scene.ppm
	echo ','
	# ./bin/c-ray-opt -s 4000x4000 -i input/scene -o output/opt_scene.ppm
	# echo ','
	./bin/c-ray-pth -s 4000x4000 -i input/scene -o output/pth_scene.ppm
	echo ','
	./bin/c-ray-omp -s 4000x4000 -i input/sphfract -o output/omp_sphfract.ppm
	echo ','
	./bin/c-ray-seq -s 4000x4000 -i input/sphfract -o output/seq_sphfract.ppm
	echo ','
	# ./bin/c-ray-opt -s 4000x4000 -i input/sphfract -o output/opt_sphfract.ppm
	# echo ','
	./bin/c-ray-pth -s 4000x4000 -i input/sphfract -o output/pth_sphfract.ppm
	echo "]}"
