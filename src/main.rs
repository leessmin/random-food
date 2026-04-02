use random_food::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    args::exec_args()?;

    Ok(())
}
