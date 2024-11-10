use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(help = "The package spec to install.")]
    pub package_spec: Option<String>,

    #[arg(short, long, default_value_t = true)]
    name: bool,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.name);
    if let Some(package_spec) = cli.package_spec {
        println!("Hello, world! {}", package_spec);
    }
}
