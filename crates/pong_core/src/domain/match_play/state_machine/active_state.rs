use super::State;

/// Only active states can be retained by a pause; a pause cannot wrap another pause.
pub(super) trait ActiveState: State {}
