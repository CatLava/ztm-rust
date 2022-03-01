

fn main() {
    let mut st = "#00cc66";
    let n: Vec<char> = st.chars()
        .collect()
        .chunks(2)
        .map(|chunk| "i");
    
    println!("{:?}", n)
}