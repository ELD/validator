use validator::Validate;

#[derive(Validate)]
//~^ ERROR: proc-macro derive panicked
//~^^ HELP: Invalid attribute #[validate] on field `s`: Validator `length` requires at least 1 argument out of `min`, `max` and `equal`
struct Test {
    #[validate(length())]
    s: String,
}

fn main() {}
