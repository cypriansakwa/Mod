fn main() {
    // Use i64 to handle large integers
    let a: i64 = 158383636738363484 +6765458262;
    let b: i64 = 7635363;

    // Compute a % b
    let result = (a % b+b)%b;//If a is negative you can modify this part as
    //let result=(a % b +b) % b//
    //This is because, for instance -4 mod 5 would be -4 and so adding b (which is 5)
    // ensures the result is non-negative. Applying the modulo operation again handles 
    //any possible overflow (this is necessary when the negative value is very large).


    println!("15 % 7 = {}", result);
}