mod mpreprocessing;
fn main(){
    let input = String::from("# random comment");
    let mut p = mpreprocessing::Mprosser::new(input.clone());
    let out = p.make();
    println!("input:\n{input}");
    println!("output:\n{out}");
}