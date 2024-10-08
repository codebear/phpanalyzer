mod array;
mod basic;
pub mod generics;
pub mod hardening;
pub mod inline;
pub mod interface;
pub mod namespace;
pub mod native;
mod objects;
pub mod phpdocs;
pub mod traversable;
pub mod try_catch;
pub mod types;
mod values;

use std::{
    ffi::OsString,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{
    analysis::{analyzer::Analyzer, state::AnalysisState},
    config::PHPAnalyzeConfig,
    issue::{Issue, IssueEmitter},
    symboldata::{FunctionData, SymbolData},
    symbols::FullyQualifiedName,
    types::union::PHPType,
    value::PHPValue,
};

pub struct TestEmitter {
    pub file_name: RwLock<Option<PathBuf>>,
    issues: Arc<RwLock<Vec<Issue>>>,
}

impl TestEmitter {
    pub fn new() -> Self {
        TestEmitter {
            file_name: RwLock::new(None),
            issues: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl IssueEmitter for TestEmitter {
    fn emit(&self, issue: Issue) {
        let start = issue.range().start_point;
        let err = issue.as_string();
        if let Some(f) = issue.filename() {
            let fname = f.to_string_lossy();
            eprintln!(
                "Issue: {:?} {}:{}:{}: {}",
                issue.severity(),
                fname,
                start.row + 1,
                start.column + 1,
                err
            );
        } else {
            eprintln!(
                "Issue: {:?} *Unknown buffer*:{}:{}: {}",
                issue.severity(),
                start.row + 1,
                start.column + 1,
                err
            );
        }
        {
            self.issues.write().unwrap().push(issue);
        }
    }
}

#[derive(Debug)]
pub struct EvaluationResult {
    pub function_data: Option<FunctionData>,
    pub return_type: Option<PHPType>,
    pub return_value: Option<PHPValue>,
    pub symbol_data: Option<Arc<SymbolData>>,
    pub issues: Vec<Issue>,
}

impl EvaluationResult {
    pub fn new() -> Self {
        Self {
            function_data: None,
            return_type: None,
            return_value: None,
            symbol_data: None,
            issues: vec![],
        }
    }
}

fn evaluate_php_buffers<T>(
    config: PHPAnalyzeConfig,
    buffers: T,
    load_native: bool,
) -> EvaluationResult
where
    T: IntoIterator<Item = (OsString, OsString)>,
{
    let emitter = TestEmitter::new();

    let symbols = Arc::new(SymbolData::new());
    if load_native {
        let mut state = AnalysisState::new_with_symbols(symbols.clone());
        crate::native::register(&mut state);
    }

    let buffers: Vec<_> = buffers.into_iter().collect();
    for (buffer_name, outer_buffer) in &buffers {
        let mut state = AnalysisState::new_with_symbols(symbols.clone());
        state.pass = 1;
        let mut analyzer =
            Analyzer::new_from_buffer(config, outer_buffer.clone(), Some(buffer_name.clone()));
        assert!(analyzer.parse(&emitter).is_ok());

        // analyzer.dump();
        analyzer.first_pass(&mut state, &emitter);
    }

    for (buffer_name, outer_buffer) in &buffers {
        let mut state = AnalysisState::new_with_symbols(symbols.clone());
        state.pass = 2;
        let mut analyzer =
            Analyzer::new_from_buffer(config, outer_buffer.clone(), Some(buffer_name.clone()));
        assert!(analyzer.parse(&emitter).is_ok());

        // analyzer.dump();
        analyzer.second_pass(&mut state, &emitter);
    }

    for idx in 0..1 {
        for (buffer_name, outer_buffer) in &buffers {
            let mut state = AnalysisState::new_with_symbols(symbols.clone());
            state.pass = 3 + idx;
            let mut analyzer =
                Analyzer::new_from_buffer(config, outer_buffer.clone(), Some(buffer_name.clone()));
            assert!(analyzer.parse(&emitter).is_ok());
            analyzer.third_pass(&mut state, &emitter);
        }
    }

    let mut result = EvaluationResult::new();
    result.symbol_data = Some(symbols);
    result.issues.clone_from(&emitter.issues.read().unwrap());
    crate::dump_missing_stats();

    result
}

fn evaluate_php_code_in_function<T: Into<OsString>>(
    config: PHPAnalyzeConfig,
    buffer: T,
) -> EvaluationResult {
    let mut outer_buffer = OsString::from("<?php ");
    outer_buffer.push("function test_output() { ");
    outer_buffer.push(buffer.into());
    outer_buffer.push("}");

    let emitter = TestEmitter::new();

    let mut analyzer = Analyzer::new_from_buffer(config, outer_buffer, Some("test_buffer".into()));
    assert!(analyzer.parse(&emitter).is_ok());

    let mut state = AnalysisState::new();
    crate::native::register(&mut state);
    // analyzer.dump();
    analyzer.first_pass(&mut state, &emitter);
    analyzer.second_pass(&mut state, &emitter);
    analyzer.third_pass(&mut state, &emitter);
    analyzer.third_pass(&mut state, &emitter);

    let mut result = EvaluationResult::new();
    let func_name = FullyQualifiedName::from("\\test_output");
    if let Ok(functions_handle) = state.symbol_data.functions.read() {
        if let Some(func_data_handle) = functions_handle.get(&func_name).cloned() {
            let func_data = func_data_handle.read().unwrap();
            result.function_data = Some(func_data.clone());
            result
                .return_type
                .clone_from(&func_data.inferred_return_type);
            result.return_value.clone_from(&func_data.return_value);
        } else {
            eprintln!("Mangler data om funksjonen test_output");
        }
    } else {
        eprintln!("Failed reading function data");
    }
    result.symbol_data = Some(state.symbol_data);
    result.issues.clone_from(&emitter.issues.read().unwrap());
    crate::dump_missing_stats();

    result
}

fn get_inferred_return_type<T: Into<OsString>>(buffer: T) -> Option<PHPType> {
    evaluate_php_code_in_function(Default::default(), buffer).return_type
}

fn get_inferred_return_value<T: Into<OsString>>(buffer: T) -> Option<PHPValue> {
    evaluate_php_code_in_function(Default::default(), buffer).return_value
}
