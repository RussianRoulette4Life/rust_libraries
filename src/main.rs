use std::io::stdin;
fn main() {
    let mut thing: String = String::new();
    stdin()
        .read_line(&mut thing)
        .expect("what");
    let finalised_input = thing.trim();
    let mut random_vector: Vec<i64> = vec![];
    for number in finalised_input.split_whitespace() {
        let mut addition: i64 = number.parse().expect("nah couldnt do it sorry, seperate numbers by white spaces only!");
        random_vector.push(addition);
    }
    let tuple_of_indexes = (0,1);
    // let changed_vector: Vec<i64> = switch_index_i64(random_vector, tuple_of_indexes);
    // println!("{:#?}", changed_vector);
    let sorted = sort_vector_i64(random_vector);
    println!("{:#?}, this one is sorted", sorted)
}
pub fn sort_vector_i64(vector_to_sort: Vec<i64>) -> Vec<i64> {
    let mut vector_copy: Vec<i64> = vec![];
    for num in &vector_to_sort {
        vector_copy.push(*num)
    }
    println!("{:#?}", vector_copy);
    let mut index: usize = 0;
    let mut prev_index: usize = 0;
    for num in &vector_to_sort{
        if index != 0 {
            if vector_to_sort[prev_index] > vector_to_sort[index] {
                vector_copy = switch_index_i64(vector_copy, (prev_index,index)); 
            }
        } else {
            println!("starting sort...")
        }
        prev_index = index;
        index += 1;

    }
    if vector_to_sort != vector_copy {
        sort_vector_i64(vector_copy)
    } else {
        return vector_copy;
    }
}
fn switch_index_i64(mut vector: Vec<i64>, indexes_to_switch: (usize,usize)) -> Vec<i64>{
    let first_index = indexes_to_switch.0;
    let second_index = indexes_to_switch.1;
    let first_num = vector[first_index];
    let second_num = vector[second_index];
    vector[second_index] = first_num;
    vector[first_index] = second_num;
    return vector;
}