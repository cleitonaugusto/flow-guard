🛡️ FlowGuardNext-Generation Adaptive Concurrency Control & Backpressure for RustCreated and Developed by: Cleiton Augusto Correa BezerraFlowGuard is a next-generation load control library for Rust. Unlike static rate limiters, FlowGuard utilizes congestion control algorithms (based on TCP Vegas) to dynamically adjust concurrency limits based on real-time latency and system health.🚀 The Innovation: Why FlowGuard?In modern distributed systems, setting a fixed limit (e.g., "max 100 connections") is a dangerous trap:Limit too high: Your system collapses under stress (Cascading Failure) before the limit is even reached.Limit too low: You waste hardware resources and reject legitimate traffic unnecessarily.FlowGuard solves this through:Auto-tuning: It monitors the RTT (Round Trip Time). If latency increases, it throttles concurrency. If the system is performing well, it expands capacity.Native Resilience: Protects your databases and upstream services from being overwhelmed.Zero-Cost Abstractions: Built with atomic operations and generic types to ensure extreme performance in Rust.🛠️ InstallationAdd this to your Cargo.toml:Ini, TOML[dependencies]
# Core version
flow-guard = "0.1.0"

# With full Axum 0.8 / Tower support
flow-guard = { version = "0.1.0", features = ["axum", "tower"] }
💻 Usage Example (Axum 0.8)FlowGuard is designed to be "plug-and-play" within the modern Rust web ecosystem.Rustuse axum::{routing::get, Router, error_handling::HandleErrorLayer};
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
                    // Automatically returns 503 Service Unavailable on overload
                    axum::response::IntoResponse::into_response(err)
                }))
                .layer(flow_layer)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
📊 The Vegas Algorithm (The Math)The Vegas strategy adjusts the concurrency limit $L$ based on the difference between the actual RTT and the base (ideal) RTT:$$diff = L \times \left(1 - \frac{RTT_{base}}{RTT_{actual}}\right)$$If $diff < \alpha$: The system is under-utilized. We increase the limit to maximize throughput.If $diff > \beta$: Congestion detected! We proactively decrease the limit before a system crash occurs.💼 Future Vision (Enterprise Version)FlowGuard follows an Open-Core model. While the community version focuses on standalone instances, our Enterprise vision includes:🌐 Distributed Flow Control: Global synchronized flow control via Redis/NATS for Kubernetes clusters.📈 Observability Dashboard: Real-time dashboards (Prometheus/Grafana) to visualize traffic throttling and latency drift.⚡ Dynamic Thresholds: Update safety policies in real-time via a centralized Control Plane without redeployment.🤝 Contact & ContributionsContributions are the heart of the Rust community! Feel free to open issues or submit pull requests.Author: Cleiton Augusto Correa BezerraEmail: augusto.cleiton@gmail.comLinkedIn: cleiton-augusto-b619435b📄 LicenseThis project is licensed under the MIT License.