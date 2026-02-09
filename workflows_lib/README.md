# Workflows Library

A library for implementing complex, intelligent workflows that orchestrate operations across multiple domain libraries.

## Overview

The `workflows_lib` provides high-level workflow implementations that compose functionality from multiple domains (tasks, git, wiki, calendar, knowledge) to accomplish complex objectives.

## Architecture

### Domain Composition

Workflows orchestrate operations across:
- **tracker_lib**: Task tracker integration
- **llm_lib**: LLM-powered analysis and generation
- **Future domains**: git, wiki, calendar, knowledge

### Workflow Types

#### 1. Daily Planning Workflow
Generates daily work plans by:
- Fetching tasks from task tracker
- Prioritizing based on due dates, priority, and context
- Optionally using LLM for intelligent recommendations
- Creating structured daily plans with time estimates

#### 2. Meeting Preparation Workflow
Prepares meeting agendas by:
- Fetching meeting details from calendar
- Gathering related tasks and context
- Using LLM to generate structured agendas
- Identifying discussion points and action items

#### 3. Status Reporting Workflow
Generates status reports by:
- Fetching completed and in-progress tasks
- Analyzing code changes (commits, PRs)
- Cross-referencing tasks with code changes
- Using LLM to generate comprehensive summaries

## Usage

### Daily Planning

```rust
use workflows_lib::workflows::daily_planning::{DailyPlanningWorkflow, DailyPlanningConfig};

#[tokio::main]
async fn main() {
    let config = DailyPlanningConfig {
        max_tasks: 10,
        use_llm: true,
        status_filter: Some(vec!["open".to_string()]),
    };

    let workflow = DailyPlanningWorkflow::new(config);
    let result = workflow.execute().await;

    if result.success {
        if let Some(plan) = result.data {
            println!("Daily plan for {}: {} tasks", plan.date, plan.tasks.len());
        }
    }
}
```

### Meeting Preparation

```rust
use workflows_lib::workflows::meeting_prep::{MeetingPrepWorkflow, MeetingPrepConfig};

#[tokio::main]
async fn main() {
    let config = MeetingPrepConfig::default();
    let workflow = MeetingPrepWorkflow::new(config);

    let result = workflow.execute("meeting-id-123").await;

    if result.success {
        if let Some(agenda) = result.data {
            println!("Agenda for {}: {} items", agenda.title, agenda.items.len());
        }
    }
}
```

### Status Reporting

```rust
use workflows_lib::workflows::status_reporting::{StatusReportWorkflow, StatusReportConfig};

#[tokio::main]
async fn main() {
    let config = StatusReportConfig {
        period: "week".to_string(),
        include_code_changes: true,
        use_llm: true,
        max_tasks: 20,
    };

    let workflow = StatusReportWorkflow::new(config);
    let result = workflow.execute().await;

    if result.success {
        if let Some(report) = result.data {
            println!("Status report: {} completed, {} in progress",
                report.completed_tasks.len(),
                report.in_progress_tasks.len());
        }
    }
}
```

## Data Models

The library provides comprehensive data models for workflow inputs and outputs:

- **DailyPlan**: Structured daily work plans with tasks and events
- **MeetingAgenda**: Meeting agendas with items and context
- **StatusReport**: Comprehensive status reports with task summaries
- **WorkflowResult**: Standardized workflow execution results with metadata

## Tracing and Observability

All workflows are instrumented with tracing for observability:

```rust
RUST_LOG=debug cargo run
```

This provides detailed logging of:
- Workflow execution start/end
- Operation durations
- Errors and warnings
- Structured context for debugging

## Development Status

This library is in active development. Current implementations provide the foundational structure with placeholder logic. Future work includes:

- [ ] Full task tracker integration
- [ ] Git history analysis
- [ ] Calendar integration
- [ ] Wiki integration
- [ ] Knowledge base integration
- [ ] Advanced LLM-powered analysis
- [ ] Task-to-commit cross-referencing
- [ ] Intelligent prioritization algorithms

## Testing

Run tests with:

```bash
cargo test -p workflows_lib
```

Run with logging:

```bash
RUST_LOG=debug cargo test -p workflows_lib -- --nocapture
```

## Contributing

When adding new workflows:

1. Create a new module under `src/workflows/`
2. Implement the workflow struct and configuration
3. Use the `#[instrument]` attribute for tracing
4. Return `WorkflowResult<T>` for consistent error handling
5. Add comprehensive tests
6. Update this README

## Best Practices

- Keep workflows focused and composable
- Use proper error handling with context
- Instrument all functions with tracing
- Write tests for all workflows
- Document configuration options
- Follow the functional programming style where appropriate
