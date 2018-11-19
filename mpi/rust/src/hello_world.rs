extern crate mpi;

use mpi::traits::*;

fn main() {
    // Initialize the MPI environment
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    // Get the number of processes
    let size = world.size();

    // Get the processes rank
    let rank = world.rank();

    // Get the processor name
    let processor_name = mpi::environment::processor_name().unwrap();

    // Print off a hello world message
    println!("Hello World | Processor {}, rank {} of {}", processor_name, rank, size);
}