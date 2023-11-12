use rand::Rng;  // for random number generation
use std::io; // for reading input


/***********************************************************************************************************/
/* Program name:  Magic_8ball.rs                                                                           */
/* Program purpose: To simulate a Magic 8ball for fun                                                      */
/* Date created:  Sunday, November 12th, 2023 (11/12/2023)                                                 */
/* Program Author: Zian Elijah Smith                                                                       */
/*                                                                                                         */
/* Made this while learning Rust                                                                           */
/***********************************************************************************************************/

    /* Responses are based on a random number */
    /* The random number will access an index */
    /* We use 0 to 20, because the array has  */
    /* 20 items, that way, all items are      */
    /* possible                               */

fn main() {


    // Define array of responses
    // Source for responses: https://en.wikipedia.org/wiki/Magic_8_Ball#Possible_answers
    let array_of_responses = [
    "It is certain.", "It is decidedly so.", "Without a doubt.", "Yes definitely.", 
    "You may rely on it.", "As I see it, yes.", "Most likely.", "Outlook good.", 
    "Yes.", "Signs point to yes.", "Reply hazy, try again.", "Ask again later.", 
    "Better not tell you now.", "Cannot predict now.", "Concentrate and ask again.",
    "Don't count on it.", "My reply is no.", "My sources say no.", 
    "Outlook not so good.", "Very doubtful. " 
     ];
     
     
    // Magic number will determine the response from the array!
    // Generate random number in the range [0, 20]
    let magic_number = rand::thread_rng().gen_range(0..20);
    
    
    // Tell user to ask question
    println!("What do you want to ask?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // print response!
    println!("{}\n\n", array_of_responses[magic_number]);


}
