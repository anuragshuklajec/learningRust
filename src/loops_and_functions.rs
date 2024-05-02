// simple for loop
fn main() {

    for i in 0..11{
        print!("{} ", i);
    }

    // Iterating over String
    let sentence = String :: from("my name is Anurag");
    let first_word = get_first_word(sentence);
    println!("First word is : {}", first_word);
}
fn get_first_word(sentence : String) -> String{
    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans

}