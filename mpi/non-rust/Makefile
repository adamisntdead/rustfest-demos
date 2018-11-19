EXECS=mpi_hello_world mpi_primes
MPICC?=mpicc

all: ${EXECS}

mpi_hello_world: hello_world.c
	${MPICC} -o hello_world hello_world.c

mpi_primes: primes.c
	${MPICC} -o primes primes.c

clean:
	rm -f ${EXECS}