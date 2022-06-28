fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("constant = {}", THREE_HOURS_IN_SECONDS);
    
    // let THREE_HOURS_IN_SECONDS = 120;
    // println!("constant = {}", THREE_HOURS_IN_SECONDS);

    // Error
    // interpreted as a constant pattern, not a new variable
    // help: introduce a variable instead: `three_hours_in_seconds_var`
 
}
