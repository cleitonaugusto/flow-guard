use flow_guard::strategy::VegasStrategy;
use flow_guard::LimitStrategy;
use std::time::Duration;

#[test]
fn test_vegas_actually_adjusts() {
    let strategy = VegasStrategy::new(10);


    assert_eq!(strategy.current_limit(), 10);


    for _ in 0..20 {
        strategy.on_success(Duration::from_millis(10));
    }
    let limit_after_fast = strategy.current_limit();
    assert!(limit_after_fast > 10, "Deveria aumentar com sistema rápido. Limite: {}", limit_after_fast);


    for _ in 0..20 {
        strategy.on_success(Duration::from_millis(200)); // 20x mais lento
    }
    let limit_after_slow = strategy.current_limit();
    assert!(limit_after_slow < limit_after_fast, "Deveria diminuir com congestionamento. Era: {}, Agora: {}", limit_after_fast, limit_after_slow);

    println!("✅ Teste passou! Limite ajustou de 10 → {} → {}", limit_after_fast, limit_after_slow);
}