//// This is a crate made as a joke to power [spaceifier-app]

pub mod utils {
    /// Takes a number and returns the amount of spaces in a string.
    /// ```
    /// use spaceifier::utils;
    /// let string = utils::gen_space(4);
    ///
    /// assert_eq!(string, "    ");
    /// ```
    pub fn gen_space(space_num: i32) -> String {
        let mut space_char_vec: Vec<char> = vec![];

        for _ in 0..space_num {
            space_char_vec.push(' ');
        }

        let space_string: String = space_char_vec.iter().collect();

        space_string
    }

    /// Takes a string and some text and inserts the string in between every character.
    /// ```
    /// use spaceifier::utils;
    ///
    /// let text = utils::add_stuff_to_string("Hello!", "-");
    ///
    /// assert_eq!(text, "H-e-l-l-o-!-");
    /// ```
    pub fn add_stuff_to_string(text: &str, addition: &str) -> String {

        let mut output_vec: Vec<String> = vec![];

        for i in text.to_string().chars() {
            output_vec.push(format!("{}{}", i, addition));
        }

        let output: String = output_vec.join("");
        
        output
    }
}

/// Takes a string and number of spaces and returns a string with that number of spaces in between
/// every character
/// ```
/// let text = spaceifier::spaceify("Hello, world!", 1);
///
/// assert_eq!(text, "H e l l o ,   w o r l d !");
/// ```
pub fn spaceify(text: &str, space_num: i32) -> String {
    
    let space_string: String = crate::utils::gen_space(space_num);
    
    let spaceified_text: String = crate::utils::add_stuff_to_string(text, &space_string);

    spaceified_text.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::spaceify;
    #[test]
    fn spaceifier_test() {
        assert_eq!(
            spaceify("Hello", 1),
            String::from("H e l l o")        
        );
    }
}
