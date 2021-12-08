use std::env;           // to pass arguments
use std::thread;       // for threads

                                          // print_partition_info
fn print_partition_info(vs: &Vec<Vec<usize>>){ 
    println!("Number of partitions = {}", vs.len());
    for i in 0..vs.len(){
        println!("\tsize of partition {} = {}", i, vs[i].len());
    }
}
   /*
  Generates data for the rest of the program
Calls generate_data() to generates a vector of numbers that serves as input for the rest of the program.
  */                                             //generate_data
fn generate_data(num_elements: usize) -> Vec<usize>{
    let mut v : Vec<usize> = Vec::new();
    for i in 0..num_elements {                         
        v.push(i);
    }
    return v;
}
    /*
    Partitions the data
Calls partition_data_in_two() which partitions the input numbers into two partitions
    */                                          //partition_data_in_two
fn partition_data_in_two(v: &Vec<usize>) -> Vec<Vec<usize>>{
    let partition_size = v.len() / 2;
    let mut vecs: Vec<Vec<usize>> = Vec::new();           //create vector to hold values
    let mut one : Vec<usize> = Vec::new();

    for i in 0..partition_size{
        one.push(v[i]);                //add one to the vector 
    }
    vecs.push(one);
    let mut two : Vec<usize> = Vec::new();

    for i in partition_size..v.len(){      //add the other hald of the values into vector two
        two.push(v[i]);
    }
    vecs.push(two);        //vecs add to the vector that is returned
    vecs                   //results of the vector
}
   /*
   Performs the map step
Calls map_data() for each of the two partitions,  which returns the sum of all the numbers in that partition.
   */                                  
fn map_data(v: &Vec<usize>) -> usize{    //to sum up values in vectors
    let mut totals = 0;
    for i in v{                     
        totals += i;
    }
    totals
}
    /*
    Performs the reduce step
Gathers the intermediate results produced by each call to map_data()
Calls reduce_data() that sums up the intermediate results produced by the map step to produce the final sum of all the input numbers.
    */                         //to sum up values in vectors
fn reduce_data(v: &Vec<usize>) -> usize{
    let mut totals = 0;
    for i in v{
        totals += i;
    }
    totals
}

fn main() {                       // main functionalitly for single threaded map reduce_data program

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("ERROR: Usage {} num_partitions num_elements", args[0]);
        return;
    }
    let num_partitions : usize = args[1].parse().unwrap();
    let num_elements : usize = args[2].parse().unwrap();
    if num_partitions < 1{
      println!("ERROR: num_partitions must be at least 1");
        return;
    }
    if num_elements < num_partitions{
        println!("ERROR: num_elements cannot be smaller than num_partitions");
        return;
    }                                                   // generate_data
    let v = generate_data(num_elements);
    let see_inside = partition_data_in_two(&v);          //partition_data_in_two
    print_partition_info(&see_inside);                   // print_partition_info

    let mut in_total : Vec<usize> = Vec::new();
                                                         //process partition
    let clone_1 = see_inside[0].clone();
    let one_thread = thread::spawn(move || map_data(&clone_1));
    let one_rust = one_thread.join().unwrap();
    in_total.push(one_rust);

    let clone_2 = see_inside[1].clone();
    let second_thread = thread::spawn(move || map_data(&clone_2));
    let second_rust = second_thread.join().unwrap();
    in_total.push(second_rust);
                                                    // process in_total to get totals
    println!("Intermediate sums = {:?}", in_total);
    let totals = reduce_data(&in_total);
    println!("Sum = {}", totals);
                                                //calling partition_data to partition data into equal partition
    let vec2 = partition_data(num_partitions, &v);
    print_partition_info(&vec2);

              // creates thread per partition to use each thread
    let mut threads : Vec<thread::JoinHandle<usize>> = Vec::new();             //vector to hold threads
    for a in 0..num_partitions {
        let data = vec2[a].clone();        // create a clone of vec2 and push 
        threads.push(thread::spawn(move || map_data(&data)));
    }
    let mut in_total2 : Vec<usize> = Vec::new();
    for thread in threads {
       let reveal = thread.join().unwrap();     //join threads to put vector in order to get a value
       in_total2.push(reveal); 
    }
    println!("Intermediate sums = {:?}", in_total2); //prints Intermediate Sum
    let second_total = reduce_data(&in_total2);     // calls reduced_data
    println!("Sum = {}", second_total);             //prints total 
}

fn partition_data(num_partitions: usize, v: &Vec<usize>) -> Vec<Vec<usize>>{
   let mut see_inside: Vec<Vec<usize>> = Vec::new();         // Vectors to hold contents
   let mut i = 0;

   let partition_size = v.len() / num_partitions;           // variables
   let mut remainder =  v.len() % num_partitions;
   let mut complete = 0;
  
   for _xn in 0..num_partitions {               //vecotor to hold contents
        let mut one : Vec<usize> = Vec::new();         // one takes all contents from vector
        for _cn in 0..partition_size {
            if i != v.len() {              // See true statement for remainder with param 
                if remainder > 0 && complete == 0 {
                    one.push(v[i]);
                    i = i + 1; 
                    remainder = remainder - 1;
                    complete = 1;
                }
            one.push(v[i]);
            i = i + 1;
            }
        }
        complete = 0;             // check for remainder after vector one and two
        see_inside.push(one);
   }
   see_inside                              //the final vector
}


/*
cited work:
Author: Exploration: Threads
Date: Fall 2021
URL: https://replit.com/@cs344/104threads
use std::thread;
use std::time::Duration;

fn count_to_val(tid: char, x: u32){
  for i in 1..x {
    println!("Thread {}, i is {}", tid, i);
    thread::sleep(Duration::from_millis(10));
  }
}

fn main(){

  let t1 = thread::spawn(|| count_to_val('a', 5));
  let t2 = thread::spawn(|| count_to_val('b', 10));
  let _r1 = t1.join().unwrap();
  let _r2 = t2.join().unwrap();
}

---------------------------------------------------------------------------
cited work:
Author: Exploration: Threads
Date: Fall 2021
URL: https://replit.com/@cs344/104threadsmoveerrrs
use std::thread;

fn find_max(xs: Vec<i32>) -> i32{
  let mut max = xs[0];
  for x in xs{
    if x > max{
      max = x;
    }
  }
  max
}

fn main(){
  let mut v : Vec<i32> = Vec::new();
  v.push(11);
  v.push(187);
  v.push(34);
  v.push(99);

  let handle = thread::spawn(move || find_max(v));
  let res = handle.join().unwrap();
  println!("The max value is {}", res);
  println!("The first element of the vector is {}", v[0]);
}
----------------------------------------------------------------------------
cited work:
Author: Exploration: Threads
Date: Fall 2021
URL: https://replit.com/@cs344/104closurers
fn main(){
  let x = 5;
    
  // We define a closure named is_divisible_by_x
  // This closure captures the value of x
  // At execution time, is_divisible_by_x takes one argument z and will divide z by the value of x
  let is_divisible_by_x = |z| {
    z % x == 0
  };

  let a = 10;
  let mut res = is_divisible_by_x(a);
  println!("is_divisible_by_x({}) returns {}", a, res);

  let b = 13;
  res = is_divisible_by_x(b);
  println!("is_divisible_by_x({}) returns {}", b, res);
}
-----------------------------------------------------------------------------------
cited work:
Author: Exploration: Threads
Date: Fall 2021
URL: https://doc.rust-lang.org/std/thread/
use std::thread;

thread::spawn(move || {
    // some work here
});
use std::thread;

let thread_join_handle = thread::spawn(move || {
    // some work here
});
// some work here
let res = thread_join_handle.join();
use std::thread;

thread::Builder::new().name("thread1".to_string()).spawn(move || {
    println!("Hello, world!");
})
use std::thread;
use std::sync::mpsc::channel;

let (tx, rx) = channel();

let sender = thread::spawn(move || {
    tx.send("Hello, thread".to_owned())
        .expect("Unable to send on channel");
});

let receiver = thread::spawn(move || {
    let value = rx.recv().expect("Unable to receive from channel");
    println!("{}", value);
});

sender.join().expect("The sender thread has panicked");
receiver.join().expect("The receiver thread has panicked");

use std::thread;

let computation = thread::spawn(|| {
    // Some expensive computation.
    42
});

let result = computation.join().unwrap();
println!("{}", result);

----------------------------------------------------------------------------------
cited work:
Author: Exploration: References
Date: Fall 2021
URL:https://doc.rust-lang.org/1.28.0/book/2018-edition/ch15-00-smart-pointers.html
All concepts were read in this exploration

-----------------------------------------------------------------------------------
cited work:
Author: Exploration: References
Date: Fall 2021
URL:https://doc.rust-lang.org/1.28.0/book/2018-edition/ch19-01-unsafe-rust.html
All concepts were read in this exploration
----------------------------------------------------------------------------------
cited work:
Author: Exploration: References
Date: Fall 2021
URL:https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html
All concepts were read in this exploration

--------------------------------------------------------------------------------
cited work:
Author: Exploration: Ownership
Date: Fall 2021
URL:https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)
All concepts were read in this exploration


*/