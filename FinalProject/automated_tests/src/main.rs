use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
struct Task {
    task_id: String,
    generated_code: String,
    test: String,
}

#[derive(Serialize)]
struct TestResult {
    task_id: String,
    result: u8,
}

fn main() {
    // Read the JSON file
    let json_data = fs::read_to_string("/home/korinlu/Dev/ku_dl/project/automated_tests/tasks.json").expect("Unable to read file");
    let tasks: Vec<Task> = serde_json::from_str(&json_data).expect("JSON was not well-formatted");

    let mut results = Vec::new();

    // Loop through each task
    for task in tasks {
        let task_id = &task.task_id;
        let generated_code = &task.generated_code;
        let test_code = &task.test;
    
        // Create a Rust file for testing
        let code = format!("{}\n\n#[cfg(test)]\nmod tests {{\n    use super::*;\n    {}\n}}\n", generated_code, test_code);
        let filename = "/home/korinlu/Dev/ku_dl/project/automated_tests/tests/task_tmp.rs";
        fs::write(&filename, code).expect(&format!("Unable to write to file {}", filename));

        // Run the tests
        let output = Command::new("cargo")
            .arg("test")
            .arg("--test")
            .arg("task_tmp")
            .output()
            .expect("Failed to execute command");

        // Check if the tests passed or failed
        let result = if output.status.success() { 1 } else { 0 };
        results.push(TestResult {
            task_id: task_id.clone(),
            result,
        });

        // Clean up the test file
        // fs::remove_file(&filename).expect("Unable to delete file");
    }

    // Write results to a new JSON file
    let results_filename = "test_results.json";
    let results_json = serde_json::to_string_pretty(&results).expect("Failed to serialize results");
    fs::write(results_filename, results_json).expect("Unable to write results file");

    println!("Results saved to {}", results_filename);
}

