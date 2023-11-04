#[macro_export]
macro_rules! assert_template_eq {
    ($str: expr, $template: expr) => {
        pretty_assertions::assert_eq!($str, $template.to_string())
    };
}
