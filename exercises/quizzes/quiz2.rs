// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn bar_loop(count: usize, mut input: String) -> String {
        for _n in 0..count {
            input = input + "bar"
        }
        input
    }

    // TODO: Complete the function as described above.
    pub fn transformer(input_commands: Vec<(String, Command)>) -> Vec<String> {
        let mut outputs = Vec::<String>::new();
        for mut input in input_commands {
            match input.1 {
                Command::Uppercase => input.0 = input.0.to_uppercase(),
                Command::Trim => input.0 = input.0.trim().to_string(),
                Command::Append(u) => input.0 = bar_loop(u, input.0),
            };
            outputs.push(input.0);
        }
        outputs
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use crate::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
