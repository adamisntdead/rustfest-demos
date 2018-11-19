extern crate mpi;

use mpi::traits::*;
use mpi::collective::SystemOperation;

const N_LO: i32 = 1;
const N_HI: i32 = 131072;
const N_FACTOR: i32 = 2;

fn main() {
    // Initialize the MPI environment
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    // Get the number of processes
    let id = world.rank();
    let p = world.size() as usize; // Number of processes

  if id == 0 {
    println!("Prime Number Counter");
    println!("  Counting the primes between {} and {}", N_LO, N_HI);
  }

  // Search the range [2, N] for primes.
  // After each search, double N and do it again.
  let mut n = N_LO;
  while n <= N_HI {
    let mut primes = 0;
    let wtime = mpi::environment::time();

    // `t` counts the primes that the process ID finds. 
    let mut t = 0;

    // Process n checks 2 + n, 2 + n + P, 2 + n + 2P, ...
    // where 0 <= n <= P - 1

    for i in ((2 + id)..(n + 1)).step_by(p) {
      let mut is_prime = 1;
      for j in 2..i {
        if i % j == 0 {
          is_prime = 0;
          break
        }
      }
      t += is_prime;
    }

    
    if id == 0 {
      world.process_at_rank(id).reduce_into_root(&t, &mut primes, SystemOperation::sum());
    } else {
      world.process_at_rank(0).reduce_into(&t, SystemOperation::sum());
    }


    // world.process_at_rank(0).reduce_into(primes)

    if id == 0 {
      println!("  {} {} {}", n, primes, mpi::environment::time() - wtime);
    }

    n *= N_FACTOR;
  }
}

// extern crate mpi;

// use mpi::traits::*;



// /// Count the number of primes between n_lo and n_hi
// fn main() {
//   let universe = mpi::initialize::unwrap();
//   let world = universe.world();

//   let id = world.rank();
//   let p = world.size(); // Number of processes

//   if id == 0 {
//     println!("Prime Number Counter");
//     println!("  Counting the primes between {} and {}", N_LO, N_HI);
//   }

//   // Search the range [2, N] for primes.
//   // After each search, double N and do it again.
//   let n = N_LO;
//   while n <= N_HI {
//     let primes = vec![];
//     let wtime = mpi::environment::time();

//     // `t` counts the primes that the process ID finds. 
//     let mut t = 0;

//     // Process n checks 2 + n, 2 + n + P, 2 + n + 2P, ...
//     // where 0 <= n <= P - 1

//     for i in ((2 + id)..(n + 1)).step_by(p) {
//       let mut is_prime = 1;
//       for j in 2..i {
//         if i % j == 0 {
//           is_prime = 0;
//           break
//         }
//       }
//       t += is_prime;
//     }
//   }
// }