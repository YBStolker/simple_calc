use super::santized_input::SanitizedInput;

pub struct Input(pub String);

impl Input {
    pub fn sanitize(self) -> SanitizedInput {
        SanitizedInput(
            self.0
                .trim()
                .replace(" ", "")
                .replace("\t", "")
                .replace("\n", "")
                .replace("()", "")
                .replace("+-", "-")
                .replace("--", "+"),
        )
    }
}
