// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
    pub change: bool,
}

impl ReportCard {
    pub fn new(grade: f32, student_name: String, student_age: u8) -> Self {
        ReportCard {
            grade,
            student_name,
            student_age,
            change: false,
        }
    }
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            &self.grade_desplay()
        )
    }

    pub fn grade_desplay(&self) -> String {
        if !self.change {
            format!("{}", self.grade)
        } else {
            let fmt = match self.grade {
                g if g >= 1.0 && g < 1.5 => "A+",
                g if g >= 1.5 && g < 2.0 => "A",
                g if g >= 2.0 && g < 2.5 => "B+",
                g if g >= 2.5 && g < 3.0 => "B",
                g if g >= 3.0 && g < 3.5 => "C+",
                g if g >= 3.5 && g < 4.0 => "D",
                g if g >= 4.0 && g < 5.5 => "F-",
                _ => "F-",
            };
            format!("{}", fmt)
        }
    }

    pub fn change_grade(&mut self, grade: f32) {
        self.change = true;
        self.grade = grade;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard::new(2.1, "Tom Wriggle".to_string(), 12);
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let mut report_card = ReportCard::new(2.1, "Gary Plotter".to_string(), 11);
        report_card.change_grade(1.2);
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
