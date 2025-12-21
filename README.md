# FlowGuard

**Adaptive Concurrency Control and Backpressure for Axum/Tower**

[![Crates.io](https://img.shields.io/crates/v/flow-guard)](https://crates.io/crates/flow-guard)
[![Documentation](https://docs.rs/flow-guard/badge.svg)](https://docs.rs/flow-guard)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Repository](https://img.shields.io/badge/github-repository-blue)](https://github.com/cleitonaugusto/flow-guard)

## The Problem: Static Limits are a Guessing Game

When building resilient microservices in Rust, setting a static concurrency limit (like `semaphore::Permits` or a fixed worker pool) is a common but fragile approach.

*   Set the limit **too high**, and a sudden spike can overwhelm your database or external API, causing a cascading failure.
*   Set it **too low**, and you're wasting resources and unnecessarily throttling valid traffic.

You're left tuning a magic number based on guesses rather than the actual health of your system.

## The Solution: Adapt Based on Latency

FlowGuard is a Tower service layer that implements **adaptive concurrency control**. Instead of a fixed limit, it dynamically adjusts the number of concurrent in-flight requests by monitoring their latency (round-trip time).

*   **When latency increases**, it reduces the concurrency limit, applying backpressure at the edge of your service.
*   **When the system is responsive**, it cautiously increases the limit to utilize available capacity.

The core algorithm is inspired by **TCP Vegas**, a congestion control algorithm known for its efficiency and low latency.

## Quick Start with Axum

Add FlowGuard to your `Cargo.toml`:
```toml
[dependencies]
flow-guard = "0.1"
Protect an Axum router in minutes:

rust
use axum::{routing::get, Router};
use flow_guard::{FlowGuardLayer, strategy::VegasStrategy};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    // Start with an initial limit of 10 concurrent requests.
    let strategy = VegasStrategy::new(10);
    let app = Router::new()
        .route("/api", get(|| async { "Hello, guarded world!" }))
        .layer(ServiceBuilder::new().layer(FlowGuardLayer::new(strategy)));

    // ... bind and serve as usual
}
When the limit is reached, the layer returns a 503 Service Unavailable response, signaling to callers (or upstream load balancers) to back off.

How It Works: The Vegas Strategy
The Vegas strategy inside FlowGuard maintains two key metrics:

Base RTT: The minimum observed round-trip time (system's healthy baseline).

Current RTT: The latency of recent requests.

The algorithm continuously compares them. If the current RTT consistently exceeds the base RTT by a certain threshold (alpha), it infers the system is congested and reduces the concurrency limit. If everything is fast, it slowly probes for more capacity.

You can tune the sensitivity:

rust
let strategy = VegasStrategy::new(10)
    .with_alpha(2)   // Lower = more sensitive to latency increases
    .with_beta(4);   // Higher = more aggressive in adding capacity
Core Features
Adaptive, Not Static: Eliminates the need for static concurrency limits.

Tower & Axum Native: Works seamlessly with the Rust service ecosystem.

Minimal Overhead: Built with performance in mind using efficient data structures.

Informative Errors: Integrates with Tower's error handling to provide clear backpressure signals.

Comparison with Similar Solutions
The Rust ecosystem has several approaches to load management. Here's how FlowGuard compares:

Aspect	FlowGuard	Static Semaphores	Rate Limiters	Queue-Based Solutions
Primary Goal	Prevent downstream overload	Limit max concurrency	Limit requests/second	Maintain target latency
Adaptive?	✅ Yes (latency-based)	❌ No (fixed)	❌ Usually static	✅ Yes (queue-based)
Protects Downstream?	✅ Proactively	⚠️ Only if limit is correct	❌ No (controls rate)	⚠️ Indirectly
Queueing	❌ No (immediate backpressure)	❌ No	❌ Usually no	✅ Yes
Configuration	Simple (initial limit)	Simple (fixed number)	Medium (RPS + burst)	Complex (queue + latency targets)
Best Use Case	Protecting DBs/APIs from overload	Simple worker pools	API rate limiting	User-facing latency SLAs
When to Choose FlowGuard
FlowGuard is particularly useful when:

Your service depends on downstream components (databases, external APIs) with variable performance

You want zero queuing and immediate backpressure signals (503 responses)

You need protection that adapts automatically without manual tuning

Simplicity matters - set an initial limit and let the algorithm adjust

When Other Approaches Might Be Better
Static semaphores work well for CPU-bound worker pools with known limits

Rate limiters (like tower-governor) are essential for API quota enforcement

Queue-based solutions (like Little Loadshedder) excel when you need predictable latency guarantees and can tolerate queueing

FlowGuard complements these approaches rather than replacing them. For example, you might use:

Rate limiting at your API gateway

FlowGuard to protect your database layer

Static semaphores for CPU-intensive background tasks

Is This Production Ready?
FlowGuard is a young crate (v0.1.x). It implements a proven algorithm, but its integration and edge cases are being refined. The current version is best suited for:

Evaluation and testing in staging environments.

Services where the primary risk is overloading a downstream dependency (like a database).

Important considerations:

The state is per-service-instance. For a cluster-wide limit, you need a distributed coordinator (a planned future feature).

Like any adaptive system, it needs traffic to "learn". Its behavior with very low traffic or bursty patterns is still being observed.

Contributions, bug reports, and real-world deployment stories are incredibly valuable to help mature this project.

Contributing
Contributions are welcome! Please feel free to submit pull requests or open issues on GitHub.

License
This project is licensed under the MIT License - see the LICENSE file for details.