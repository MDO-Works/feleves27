pub struct Kutya {
    nev: String,
    kor: i8,
}

impl Kutya {
    pub fn new(nev: String, kor: i8) -> Self {
        Self { nev, kor }
    }

    pub fn set_nev(&mut self, nev: String) {
        self.nev = nev;
    }

    pub fn get_nev(&self) -> &str {
        &self.nev
    }

}