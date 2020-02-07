enum Week {
    WorkDay,
    Weekend,
}

fn main() {
    let day = Week::Weekend;
    match day {
        Week::Weekend => println!("休息"),
        _ => println!("干活"),
    }
    if let Week::Weekend = day {
        println!("休息")
    } else {
        println!("干活")
    }
}
