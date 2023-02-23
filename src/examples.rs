///
/// Example struct of a serializable person
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ExamplePerson {
    name: String,
    age: i64,
    occupation: String,
}

pub mod json {
    use super::ExamplePerson;

    const FILE_PATH: &str = "./out/_example_01.json";

    ///
    /// Example function of writing a JSON file
    ///
    pub fn write_file() {
        let john = ExamplePerson {
            name: String::from("John Doe"),
            age: 43,
            occupation: String::from("Fire Fighter"),
        };

        std::fs::write(FILE_PATH, serde_json::to_string_pretty(&john).unwrap()).unwrap()
    }

    ///
    /// Example function of reading a JSON file
    ///
    pub fn read_file() {
        let contents = std::fs::read_to_string(FILE_PATH).unwrap();
        let example_person: ExamplePerson = serde_json::from_str(&contents).unwrap();
        println!("The contents of '{}' are {:?}", FILE_PATH, example_person);
    }
}

pub mod csv {
    use super::ExamplePerson;

    const FILE_PATH: &str = "./out/_example_02.csv";

    ///
    /// Example function of writing a CSV file
    ///
    pub fn write_file() {
        let mut writer = csv::Writer::from_path(FILE_PATH).unwrap();
        let john = ExamplePerson {
            name: String::from("John Doe"),
            age: 43,
            occupation: String::from("Fire Fighter"),
        };
        writer.serialize(john).unwrap();
        writer.flush().unwrap();
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Score {
        id: String,
        score: i64,
    }

    ///
    /// Example function of reading a CSV file
    ///
    pub fn read_file() {
        let mut reader = csv::Reader::from_path(FILE_PATH).unwrap();
        let mut iter = reader.deserialize();
        if let Some(result) = iter.next() {
            let example_person: ExamplePerson = result.unwrap();
            println!("The first row of '{}' is {:?}", FILE_PATH, example_person);
        }
    }
}
