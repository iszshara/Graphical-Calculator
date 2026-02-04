use gcalc::tokenizer::Lexer;

fn main() 
{
   println!("Please enter your term: ");

   let mut input = String::new();
   std::io::stdin()
      .read_line(&mut input)
      .expect("Can not read user input");

   let mut lexer = Lexer::new(&mut input);
   let tokens = lexer.tokenize();

   println!("Tokenisierte Ausgabe: ");
   for token in tokens
   {
      println!("{:?}", token);
   }
}
