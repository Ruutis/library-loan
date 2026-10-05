// Loan
struct BookLoan {
    title: String,
    borrower: String,
    days_remaining: u32,
}

// impl
impl BookLoan {
//Oletus päivät 14
    fn new(title: String, borrower: String) -> BookLoan {
        BookLoan {
            title,
            borrower,
            days_remaining: 14,
        }
    }

    fn summary(&self) {
        println!(
            "{} is borrowed by {} for {} more days.",
            self.title, self.borrower, self.days_remaining
        );
    }

    //muuttuva
    fn pass_day(&mut self) {
        if self.days_remaining > 0 {
            self.days_remaining -= 1;
        }
    }
}

// Pääosa
fn main() {
    let mut loan = BookLoan::new(String::from("The Rust Book"), String::from("Iines"));
//Tulostus
    loan.summary();

    loan.pass_day();
    loan.pass_day();
    loan.pass_day();

// Tulostus päivitetty
    loan.summary();
}