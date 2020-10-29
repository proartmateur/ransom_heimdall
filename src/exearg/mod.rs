use std::env;

pub fn get_args() -> Option<Vec<String>> {
    let mut args: Vec<String> = env::args().collect();
    println!("ARGS: {:?}", args);
    println!("LEN: {:?}", args.len());
    if args.len() <= 1 {
        println!("No se encontraron archivos para transcodificar. ಥ_ಥ ");
        return None;
    }
    args.remove(0);
    return Some(args);
}