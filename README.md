🛡️ FlowGuard: Next-Generation Adaptive Concurrency Control & Backpressure for Rust
Crates.io | License: MIT | Rust

🎯 About the Project
Created and developed by: Cleiton Augusto Correa Bezerra

FlowGuard is a next-generation load control library. Unlike static rate limiters, FlowGuard uses congestion control algorithms (TCP Vegas) to dynamically adjust load limits based on real latency and system health.

🚀 The Innovation: Why FlowGuard?
Setting a fixed limit (e.g., "maximum 100 connections") is a trap in modern systems:

Limit too high: System crashes (Cascading Failure) before reaching the limit

Limit too low: Wasted hardware and refusal of legitimate traffic

FlowGuard solves this with:

✅ Auto-tuning:
Observes RTT (Round Trip Time). If latency rises, it reduces concurrency. If the system is fast, it expands capacity.

✅ Native Resilience:
Protects databases and external services from overload.

✅ Zero-Cost Abstractions:
Built with atomic operations in Rust for extreme performance.

📦 Installation
Add this to your Cargo.toml:

toml
[dependencies]
# Core Version
flow-guard = "0.1.0"

# With full Axum 0.8 / Tower support
flow-guard = { version = "0.1.0", features = ["axum", "tower"] }
🚀 Quick Start (Axum 0.8)
FlowGuard is plug-and-play and uses the modern Rust middleware pattern.

rust
use axum::{routing::get, Router, error_handling::HandleErrorLayer};
use flow_guard::{FlowGuardLayer, VegasStrategy, FlowError};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
// Initialize: (Initial Limit, Minimum, Maximum)
let strategy = VegasStrategy::new(10, 2, 100);
let flow_layer = FlowGuardLayer::new(strategy);

    let app = Router::new()
        .route("/api/data", get(|| async { "Hello from Protected API!" }))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: FlowError<std::convert::Infallible>| async move {
                    // Automatically returns 503 Service Unavailable if overloaded
                    axum::response::IntoResponse::into_response(err)
                }))
                .layer(flow_layer)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
📊 The Vegas Algorithm (The Math)
The Vegas strategy adjusts the limit $L$ based on the difference between current RTT and base RTT:

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
Built-in metrics collection

📚 Documentation
Full API documentation is available on docs.rs

🤝 Contributing
Contributions are the heart of the Rust community! Feel free to submit pull requests or open issues.

Author: Cleiton Augusto Correa Bezerra
Email: augusto.cleiton@gmail.com
LinkedIn: cleiton-augusto-b619435b

📄 License
This project is licensed under the MIT License - see the LICENSE file for details.

Made with ❤️ and Rust