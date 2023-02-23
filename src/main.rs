mod examples;

fn main() {
    // JSON examples
    examples::json::write_file();
    examples::json::read_file();

    // CSV examples
    examples::csv::write_file();
    examples::csv::read_file();
}
