🛡️ FlowGuard: Next-Generation Adaptive Concurrency Control & Backpressure for Rust
[![Crates.io](https://img.shields.io/crates/v/flow-guard.svg)](https://crates.io/crates/flow-guard)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Documentation](https://docs.rs/flow-guard/badge.svg)](https://docs.rs/flow-guard)

## 🎯 About the Project
Created and developed by: Cleiton Augusto Correa Bezerra

FlowGuard is a next-generation load control library. Unlike static rate limiters, FlowGuard uses congestion control algorithms (TCP Vegas) to dynamically adjust load limits based on real latency and system health.

## 🚀 The Innovation: Why FlowGuard?
Setting a fixed limit (e.g., "maximum 100 connections") is a trap in modern systems:

- **Limit too high**: System crashes (Cascading Failure) before reaching the limit
- **Limit too low**: Wasted hardware and refusal of legitimate traffic

FlowGuard solves this with:

### ✅ Auto-tuning:
Observes RTT (Round Trip Time). If latency rises, it reduces concurrency. If the system is fast, it expands capacity.

### ✅ Native Resilience:
Protects databases and external services from overload.

### ✅ Zero-Cost Abstractions:
Built with atomic operations in Rust for extreme performance.

### ✅ Dynamic Semaphore Adjustment:
Unlike static implementations, FlowGuard's semaphore adjusts in real-time with the Vegas algorithm.

## 📦 Installation
Add this to your `Cargo.toml`:

toml
[dependencies]
# Core Version
flow-guard = "0.2.1"

# With Axum 0.8 / Tower support
flow-guard = { version = "0.2.1", features = ["axum", "tower"] }
🚀 Quick Start
Basic Usage
rust
use flow_guard::{FlowGuard, VegasStrategy};
use std::time::Duration;
use tokio::time::sleep;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create Vegas strategy with initial limit of 10
    let strategy = Arc::new(VegasStrategy::new(10));
    let guard = FlowGuard::new(Arc::clone(&strategy));
    
    println!("Initial limit: {}", guard.current_limit());
    println!("Available permits: {}", guard.available_permits());
    
    // Use FlowGuard to execute tasks with adaptive backpressure
    let result = guard.run(async {
        sleep(Duration::from_millis(100)).await;
        Ok::<_, &str>("Task completed successfully!")
    }).await;
    
    match result {
        Ok(msg) => println!("✅ {}", msg),
        Err(err) => println!("❌ Error: {:?}", err),
    }
    
    println!("Final limit: {}", guard.current_limit()); // Adjusted dynamically!
}
With Axum 0.8
rust
use axum::{routing::get, Router, error_handling::HandleErrorLayer};
use flow_guard::{FlowGuardLayer, VegasStrategy, FlowError};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    // Initialize with initial limit
    let strategy = VegasStrategy::new(10);
    let flow_layer = FlowGuardLayer::new(strategy);

    let app = Router::new()
        .route("/api/data", get(|| async { "Hello from Protected API!" }))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: FlowError<std::convert::Infallible>| async move {
                    // Automatically returns 503 Service Unavailable if overloaded
                    err.into_response()
                }))
                .layer(flow_layer)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
📊 The Vegas Algorithm
FlowGuard implements TCP Vegas congestion control algorithm that adjusts the concurrency limit based on the difference between expected and actual throughput:

Increases limit when system has spare capacity

Decreases limit when latency indicates congestion

Self-tuning based on real-time performance metrics

🔧 Features
✅ Dynamic Adaptation
Real-time concurrency adjustment based on system health

Proactive congestion prevention

No manual tuning required

✅ Resilience Patterns
Protects against cascading failures

Preserves system stability under load

Graceful degradation

✅ Production Ready
Built with atomic operations for maximum performance

Zero-cost abstractions

Seamless integration with Axum/Tower ecosystem

✅ Observability
Built-in metrics: current_limit() and available_permits()

Dynamic semaphore adjustment in real-time

📚 Documentation
Full API documentation is available on docs.rs

🎯 Examples
Check the examples/ directory:

basic_usage.rs - Basic FlowGuard usage

server_demo.rs - Axum web server example (requires axum feature)

Run examples with:

bash
cargo run --example basic_usage
cargo run --example server_demo --features axum,tower
📋 Changelog
v0.2.1 (2024-12-27)
Fixed
Implement dynamic semaphore adjustment (was static in v0.2.0)

Replace tokio::sync::Semaphore with custom DynamicSemaphore

Fix Vegas strategy integration with semaphore limits

Add observability methods: current_limit() and available_permits()

Breaking Changes
None (API compatible with v0.2.0)

v0.2.0 (2024-12-27)
Initial public release

Vegas congestion control algorithm

Basic backpressure implementation

Axum/Tower middleware support

🤝 Contributing
Contributions are the heart of the Rust community! Feel free to submit pull requests or open issues.

Please ensure:

Code follows Rust formatting standards (cargo fmt)

No clippy warnings (cargo clippy)

Tests pass (cargo test)

📄 License
This project is licensed under the MIT License - see the LICENSE file for details.

Author: Cleiton Augusto Correa Bezerra
Email: augusto.cleiton@gmail.com
LinkedIn: cleiton-augusto-b619435b

Made with ❤️ and Rust
![CI Status](https://github.com/cleitonaugusto/flow-guard/actions/workflows/ci.yml/badge.svg)
# CI Test - seg 29 dez 2025 11:54:29 -04
