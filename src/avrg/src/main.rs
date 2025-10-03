use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sum: f32 = 0.0;

    if args.len() > 1 {
        for inx in 1..args.len() {
            sum += args[inx].parse::<f32>().expect("Not a valit value");
        }

        let res: f32 = sum / ((args.len()-1) as f32);
        println!("{}", res)
    }
}
