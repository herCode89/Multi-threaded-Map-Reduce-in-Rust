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
