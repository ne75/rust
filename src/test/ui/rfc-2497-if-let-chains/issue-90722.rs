// check-pass

fn main() {
    let x = Some(vec!["test"]);

    if let Some(v) = x && v.is_empty() {
        println!("x == Some([])");
    }
}
