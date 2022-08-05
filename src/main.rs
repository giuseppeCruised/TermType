use rand::Rng;
use std::io;

fn main() {
    let mut dictionary_content:Vec<String> = Vec::new();
    dictionary_content.push("again".to_string());

    let dic = Dictionary {name: "YEAH".to_string(), content:(dictionary_content)};

    run_app(&dic,3);
}

fn run_app(dictionary: &Dictionary, sentence_length: usize){
    loop {
        let sentence = create_sentence(dictionary, sentence_length);
        print_sentence(&sentence);
        let user_sentence_as_string = get_input_as_string();
        println!("{}",user_sentence_as_string);
        if user_sentence_as_string == String::from("exit") {
            println!("TermType Ended");
            break;
        }
        match compare_sentence_with_String(&sentence, &user_sentence_as_string){
            true => println!("YES"),
            false => println!("NOOO"),
        }
        continue;
    }
}

fn get_input_as_string() -> String{
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf){
        Ok(_) => (),
        Err(error) => println!("Error: {}",error),
    }

    return buf;
}

fn print_sentence(sen: &Sentence) {
    for word in &sen.words{
        print!("{} ",word);
    }
    println!("");
}

fn create_sentence(dic: &Dictionary, sentence_length: usize) -> Sentence {
    let mut sentence: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..sentence_length{
        let dic_size = dic.content.len();
        let random_dic_index = rng.gen_range(0..dic_size);
        let random_word = dic.content[random_dic_index].clone();
        sentence.push(random_word);
    }

    return Sentence { words: (sentence) }
}

fn compare_sentence_with_String(sentence: &Sentence,string: &String) -> bool{
    return Iterator::eq(string.split(" "),sentence.words.iter());
}

struct Sentence {
    words: Vec<String>,
}

struct Dictionary{
    name: String,
    content: Vec<String>
}