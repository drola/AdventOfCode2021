
fn print_map(map: &Vec<Vec<Option<SeaCucumber>>>) {
    for row in map {
        for sw in row {
            match sw {
                Some(SeaCucumber::Eastbound) => print!(">"),
                Some(SeaCucumber::Southbound) => print!("v"),
                _ => print!("."),
            }
        }
        println!("");
    }
}
