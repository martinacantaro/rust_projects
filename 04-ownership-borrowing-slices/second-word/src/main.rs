fn main() {
    use std::io::{stdin,stdout,Write};
    let mut sentence=String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut sentence).expect("Did not enter a correct string");
    if let Some('\n')=sentence.chars().next_back() {
        sentence.pop();
    }
    if let Some('\r')=sentence.chars().next_back() {
        sentence.pop();
    }

    fn second_word(s:&String) -> &str { //function that returns the second word in a sentence
        let bytes = s.as_bytes();

        let mut space = Vec::new(); //stores the indexes where there's a space in the string (marking the end of a word)

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                space.push(i); 
            }
            if space.len() == 2 {
                return &s[space[0]+1..space[1]];
            }
        } 

        return &s[space[0]+1..s.len()];
    }

    let result = second_word(&sentence);
    println!("The second word in '{}' is '{}'", sentence, result);
}
