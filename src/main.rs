use bday_ranker::{fill_date_vec, Date};
use rand::{seq::SliceRandom, thread_rng};
use std::{fs::{File, DirBuilder}, io::Write};

fn print_welcome() {
    print!("Input the year for ranking: ");
}

fn main() -> std::io::Result<()> {
    print_welcome();

    match std::io::stdout().flush() {
        Ok(_) => (),
        Err(_) => panic!("Unable to flush standard output."),
    }

    let mut read_buffer = String::new();
    std::io::stdin()
        .read_line(&mut read_buffer)
        .expect("Cannot read line.");

    let input_year: u16 = read_buffer
        .trim()
        .parse::<u16>()
        .expect("Cannot convert to unsigned 16-bit integer.");
    let mut date_vec: Vec<Date> = Vec::new();
    fill_date_vec(&mut date_vec, input_year);
    date_vec.shuffle(&mut thread_rng());

    let mut contents: Vec<String> = Vec::new();
    let path = "out/".to_string();
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .expect("Cannot create directory.");
    let mut file =
        File::create(format!("out/{}.txt", input_year))
            .expect("Unable to create/write to file.");
    for i in 0..date_vec.len() {
        contents.push(format!(
            "Rank {}: {}.\n",
            i + 1,
            date_vec[i].get_date_string()
        ));

    }
    for buf in contents {
        write!(file, "{}", buf).expect("Cannot write to file.");
    }
    Ok(())
}
