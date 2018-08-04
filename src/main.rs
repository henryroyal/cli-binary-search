fn main() {
    let one = read_input();
    let two = read_input();

    let result = binary_search(one, two);
    for item in result {
        print!("{} ", item)
    }
}

/*
Inputs are two lines of whitespace delimited integers.
The first line consists of distinct positive integers in increasing order.
The second line consists of positive integers.
These inputs have been converted to vectors.

In both cases, the zero-index value represents the length of the remaining array.
For each item in the second vector, return the index of that item within the first.
If the item in question is absent from the first vector, return -1.
*/
fn binary_search(mut one: Vec<i64>, mut two: Vec<i64>) -> Vec<i64>
{
    // set up output vector, remove size descriptors
    let DEBUG: bool = false;
    let mut output: Vec<i64> = Vec::new();
    let len_one = one.remove(0);
    let len_two = two.remove(0);
    // assert both vectors are of equal length
    if len_one != len_two {
        return output;
    }

    let mut ix: usize = 0;
    while ix < one.len() {
        let query = two[ix];

        let mut lower_bound = 0;
        let mut midpoint = two.len() / 2;
        let mut upper_bound = two.len();
        loop {
            if DEBUG { println!("q: {}\tub: {}\tlb:{}\tmp:{}", query, upper_bound, lower_bound, midpoint) }

            // if query is midpoint, add midpoint
            if query == one[midpoint] {
                output.push(midpoint as i64);
                break;
            }

            // if query is less than midpoint,
            // set upper bound to midpoint,
            // set midpoint to upper bound / 2
            // continue
            if query < one[midpoint] {
                if upper_bound == midpoint {
                    output.push(-1);
                    break;
                }
                upper_bound = midpoint;
                midpoint = upper_bound / 2;
            }

            // if query is greater than midpoint
            // set lower bound to midpoint
            // set midpoint to midpoint / 2
            // continue
            if query > one[midpoint] {
                if lower_bound == midpoint {
                    output.push(-1);
                    break;
                }

                lower_bound = midpoint;
                midpoint = (upper_bound + lower_bound) / 2;
            }
        }

        ix += 1;
    }

    return output;
}


fn read_input() -> Vec<i64>
{
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buffer);
    buffer = buffer.replace("\n", "");
    let mut line: Vec<i64> = Vec::new();
    for item in buffer.split_whitespace() {
        let number: i64 = item.parse().unwrap();
        line.push(number);
    }

    return line;
}
