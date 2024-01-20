use crate::parser::Module;

pub fn fully_type(root: Module) {}

#[cfg(test)]
mod tests {
    #[test]
    pub fn types_simple_declarations() {}

    #[test]
    pub fn types_function_with_reference() {}

    #[test]
    pub fn types_parameter() {}

    #[test]
    pub fn types_post_declarations() {}

    #[test]
    pub fn reports_type_parameter_mismatch() {}

    #[test]
    pub fn reports_step_with_wrong_type() {}

    #[test]
    pub fn reports_array_with_wrong_type() {}
}
