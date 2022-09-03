fn main() {
    let sentence = String::from("hello world");

    fn second_word(s:&String) -> &str { //function that returns the second word in a sentence
        let bytes = s.as_bytes();

        let space = (); //stores the indexes where there's a space in the string (marking the end of a word)

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                space.push(i); //i is a tuple??? 
            }
            if space.len() == 2 {
                return &s[space.0..space.1];
            }
        } 
        return &s[space.0..s.len()];
    }

    let result = second_word(&sentence);
    println!("The second word in {} is {}", sentence, result);
}
