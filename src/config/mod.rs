#[derive(Debug)]
enum Environment {
    Dev,
    Stage,
    Prod,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Dev
    }
}

//TODO Parse
