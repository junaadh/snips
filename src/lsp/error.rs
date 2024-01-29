#[allow(clippy::enum_variant_names)]
pub enum SnipError {
    // SnipSuccess = 0,
    SnipTooFewArguments = 1,
    SnipUnableToReadFile = 2,
    SnipFailParseJson = 3,
    SnipJsonKeyNotFound = 4,
    SnipFailParseSnippet = 5,
    SnipNotFound = 6,
}
