pub struct TestCase<O> {
    pub input: String,
    pub expected_output: Option<O>,
}

impl<O> TestCase<O> {
    fn process_input(input: impl Into<String>) -> String {
        let input_str = input.into();
        if input_str.starts_with("file:") {
            let file_path = &input_str[5..];
            std::fs::read_to_string(file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path))
        } else {
            input_str
        }
    }

    pub fn new(input: impl Into<String>, expected_output: O) -> Self {
        TestCase {
            input: Self::process_input(input),
            expected_output: Some(expected_output),
        }
    }

    pub fn print_only(input: impl Into<String>) -> Self {
        TestCase {
            input: Self::process_input(input),
            expected_output: None,
        }
    }
}

pub fn read_input(input: &str) -> String {
    println!("Input: {}", input);
    if input.starts_with("file:") {
        let file_path = &input[5..]; // Skip the "file:" prefix
        std::fs::read_to_string(file_path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path))
    } else {
        input.to_string()
    }
}

pub fn compare_output<T: AsRef<str>>(expected: T, actual: T) -> bool {
    expected.as_ref() == actual.as_ref()
}

pub fn run_test_cases<F, O>(test_fn: F, test_cases: &[TestCase<O>])
where
    F: Fn(&str) -> O,
    O: PartialEq + std::fmt::Debug,
{
    for case in test_cases {
        let actual_output = test_fn(&case.input);
        match &case.expected_output {
            Some(expected) => assert_eq!(*expected, actual_output, "Failed with output: {:?}", actual_output),
            None => println!("Result: {:?}", actual_output),
        }
    }
}
