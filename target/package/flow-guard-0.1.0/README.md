🛡️ FlowGuardNext-Generation Adaptive Concurrency Control & Backpressure for RustCriado e Desenvolvido por: Cleiton Augusto Correa BezerraO FlowGuard é uma biblioteca de próxima geração para controle de carga. Ao contrário de limitadores de taxa (rate limiters) estáticos, o FlowGuard utiliza algoritmos de controle de congestionamento (TCP Vegas) para ajustar dinamicamente os limites de carga baseando-se na latência real e na saúde do sistema.🚀 A Inovação: Por que o FlowGuard?Configurar um limite fixo (ex: "máximo 100 conexões") é uma armadilha em sistemas modernos:Limite muito alto: O sistema entra em colapso (Cascading Failure) antes de atingir o limite.Limite muito baixo: Desperdício de hardware e recusa de tráfego legítimo.O FlowGuard resolve isso com:Auto-tuning: Observa o RTT (Round Trip Time). Se a latência sobe, ele reduz a concorrência. Se o sistema está rápido, ele expande a capacidade.Resiliência Nativa: Protege bancos de dados e serviços externos de sobrecarga.Zero-Cost Abstractions: Construído com operações atômicas em Rust para performance extrema.🛠️ InstalaçãoAdicione ao seu Cargo.toml:Ini, TOML[dependencies]
# Versão Core
flow-guard = "0.1.0"

# Com suporte total a Axum 0.8 / Tower
flow-guard = { version = "0.1.0", features = ["axum", "tower"] }
💻 Exemplo de Uso (Axum 0.8)O FlowGuard é plug-and-play e utiliza o padrão moderno de middlewares do Rust.Rustuse axum::{routing::get, Router, error_handling::HandleErrorLayer};
use flow_guard::{FlowGuardLayer, VegasStrategy, FlowError};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
// Inicializa: (Limite Inicial, Mínimo, Máximo)
let strategy = VegasStrategy::new(10, 2, 100);
let flow_layer = FlowGuardLayer::new(strategy);

    let app = Router::new()
        .route("/api/data", get(|| async { "Hello from Protected API!" }))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: FlowError<std::convert::Infallible>| async move {
                    // Retorna 503 Service Unavailable automaticamente se houver sobrecarga
                    axum::response::IntoResponse::into_response(err)
                }))
                .layer(flow_layer)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
📊 O Algoritmo Vegas (The Math)A estratégia Vegas ajusta o limite $L$ baseado na diferença entre o RTT atual e o RTT base:$$diff = L \times \left(1 - \frac{RTT_{base}}{RTT_{actual}}\right)$$Se $diff < \alpha$: O sistema está ocioso. Aumentamos o limite para aproveitar os recursos.Se $diff > \beta$: Congestionamento detectado! Reduzimos o limite proativamente antes do crash.💼 Visão de Futuro (Versão Enterprise)O FlowGuard é um projeto Open-Core. Enquanto a versão comunitária foca em instâncias isoladas, a nossa versão Enterprise foca em:🌐 Distributed Flow Control: Controle de fluxo global sincronizado via Redis/NATS para clusters Kubernetes.📈 Observability Dashboard: Painéis em tempo real (Prometheus/Grafana) para visualizar o estrangulamento de tráfego.⚡ Dynamic Thresholds: Alteração de políticas de segurança em tempo real via Control Plane.🤝 Contato & ContribuiçõesContribuições são o coração da comunidade Rust!Autor: Cleiton Augusto Correa BezerraEmail: augusto.cleiton@gmail.comLinkedIn: cleiton-augusto-b619435b📄 LicençaEste projeto está licenciado sob a Licença MIT.