CC=gcc-8
FLAGS_OMP=-fopenmp -O3
FLAGS_SEQ=-O3
G=

bench:
	#$(CC) $(G) -c c/bench.c -o bench.o
	#$(CC) $(G) -c c/bench.c -o bench_omp.o -fopenmp
	$(CC) $(G) -c src/main.c -o main.o
	$(CC) $(G) -c src/strassen.c -o strassen.o
	$(CC) $(G) -c $(FLAGS_OMP) src/main.c -o main-omp.o
	$(CC) $(G) -c $(FLAGS_OMP) src/strassen.c -o strassen-omp.o
	


build: bench
	$(CC) $(G) src/strassen-seq.c main.o strassen.o $(FLAGS_SEQ) -o bin/strassen-seq
	$(CC) $(G) src/strassen-task.c main-omp.o strassen-omp.o $(FLAGS_OMP) -o bin/strassen-task
	$(CC) $(G) src/strassen-task-dep.c main-omp.o strassen-omp.o $(FLAGS_OMP) -o bin/strassen-task-dep

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
