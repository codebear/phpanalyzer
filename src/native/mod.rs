use crate::analysis::state::AnalysisState;

pub mod php_8_3;

pub fn register(state: &mut AnalysisState) {
    // FIXME this could be a configuration-directive
    php_8_3::register(state);
}
