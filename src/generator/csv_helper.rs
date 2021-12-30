use csv::StringRecord;

pub trait Throw {
    fn get_throw<'a>(&self, index: usize) -> Result<&str, String>;
}

impl Throw for StringRecord {
    fn get_throw<'a>(&self, index: usize) -> Result<&str, String> {
        match self.get(index) {
            Some(value) => Ok(value),
            None => Err(format!("The index [{}] was not found", index)),
        }
    }
}
