 /* 
 This lower part is from chapter 8 of the Rust tutorial.  I am learning about hashmaps and wanted to see what the error was going to be.
 No error was generated and looking at the tutorial, there is no crab with the error symbol.  So perhaps there is no error associated
 to the code below.
  */
/* 
  fn main() {

    use std::collections::HashMap;
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    
    let text = "hello world wonderful world";
    
        let mut map = HashMap::new();
    
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    
        println!("{:?}", map);
    } */

/* Please ignore the upper part of this file.  That was from chapter 8 of the Rust tutorial.  The vector project starts below this comment block.  
The vector project will allow the user to input a list of integers, up to 10 numbers, and the program will sort the numbers from least to greatest, 
display the sorted values, determine the median of the values, display the median, and determine the mode, if one exists, and display the mode.
This project is from the end of chapter 8 of the Rust tutorial and is a test of my skills learned so far.  I am using my previous work to help 
develop this project.
 */

 use std::collections::HashMap;
 use std::io;

 fn main() {
    println!("This program will do the following:
    1. It will allow the user to enter any quantity of numbers, any order will work
    2. It will display the sorted values from least to greatest
    3. It will calculate and display the median value of the list of numbers entered
    4. It will determine and display the mode, if one exists.");

    let mut input_vector: Vec<i32> = Vec::new(); //creating a new vector called input_vector
    println!("Please enter a number for the list or enter a non-number value if no more values are desired to be entered:");
    loop {
        let mut input_value = String::new();

        println!("number: ");

        io::stdin().read_line(&mut input_value).expect("Failed to read the line");
        //using standard input to allow the user to enter any number or x as the instructions indicate

        let input_value: i32 = match input_value.trim().parse() { //Trying to repeat the input so the user can add more numbers to the vector
            Ok(num) => num, //if a number is entered go to input_vector.push line
            //Ok(x) => break, //if x is entered, break the loop; I was unable to get this to work.
            Err(_) => break, //if something else is entered repeat the loop
        };
        input_vector.push(input_value); //This adds the value from the input to the vector and loops back until the user
        //inputs a value that is not a number
    };

    determine_median(&mut input_vector); //I am using a mutable reference because the math is going to change elements of the vector
    determine_mode(&input_vector); //I do not need a mutable reference because I am mapping the values and counting how many of each are present
    
 }

/* fn sort_vector(vector: &mut Vec<i32>) {
    vector.sort()
} */

fn determine_median(vector: &mut Vec<i32>) -> i32 {

    vector.sort(); //sort the input vector
    //The vector.sort() line simply sorts the vector from smallest to largest.  There is no need to 
    //assign the sorted vector to a new variable.
    println!("The sorted vector is: {:?}", vector);

    let middle = vector.len() / 2; //finding the middle of the data points

    /* if the length of the vector evenly divides by 2, then the median is the average of the two middle data points*/
    if vector.len() % 2 == 0 { 
        let median = (vector[middle + 1] + vector[middle]) / 2;
        println!("The median is {}", median);
        median
    } else {
        let median = vector[middle] +1; //due to Rust rounding down, I add 1 to the median to get to the true median
        println!("The median is {:?}", median);
        median
    }
}

fn determine_mode(vector: &Vec<i32>) {
    
    let mut map = HashMap::new();

    for number in vector {
        let count = map.entry(number).or_insert(0);
        *count += 1; //Need to have a dereferenced count.  It is explained in detail in chapter 8.
    }

    println!("The mode is: {:?}", map);
}