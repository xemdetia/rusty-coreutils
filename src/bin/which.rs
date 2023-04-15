use argh::FromArgs;
use which::which;

#[derive(FromArgs, PartialEq)]
/// Params just grabs the first parameter
struct Params {
    #[argh(positional)]
    program: String,
}

fn main() {
    let params: Params = argh::from_env();
    let result = which(params.program).unwrap();
    println!("{}", result.into_os_string().into_string().unwrap())
}
