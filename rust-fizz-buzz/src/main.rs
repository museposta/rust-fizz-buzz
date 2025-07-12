use std::io::{self, BufRead};

/*
 * Complete the 'fizzBuzz' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn fizzBuzz(n: i32) {
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),  // Multiple of both 3 and 5
            (0, _) => println!("Fizz"),      // Multiple of 3 only
            (_, 0) => println!("Buzz"),      // Multiple of 5 only
            _ => println!("{}", i),          // Not a multiple of 3 or 5
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    fizzBuzz(n);
}

// // chsarp version
// using System;

// class Solution
// {
//     static void FizzBuzz(int n)
//     {
//         for (int i = 1; i <= n; i++)
//         {
//             if (i % 3 == 0 && i % 5 == 0)
//             {
//                 Console.WriteLine("FizzBuzz");
//             }
//             else if (i % 3 == 0)
//             {
//                 Console.WriteLine("Fizz");
//             }
//             else if (i % 5 == 0)
//             {
//                 Console.WriteLine("Buzz");
//             }
//             else
//             {
//                 Console.WriteLine(i);
//             }
//         }
//     }

//     static void Main(string[] args)
//     {
//         int n = Convert.ToInt32(Console.ReadLine().Trim());
//         FizzBuzz(n);
//     }
// }