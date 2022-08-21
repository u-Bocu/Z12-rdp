use z12_rdp_lib::tokenizer::Tokenizer;
pub(crate) fn main() -> ! {
    let mut line = String::new();

    loop {
        println!("Z1² :> ");

        std::io::stdin().read_line(&mut line).unwrap();
        let mut __tokenizer: Tokenizer = Tokenizer::new(line.clone());
        let tokens = __tokenizer.create_tokens();

        for token in tokens {
            println!("{:?}", token);
        }

        line.clear();
    }
}
