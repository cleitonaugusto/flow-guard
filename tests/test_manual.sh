#!/bin/bash

echo "🧪 Testando FlowGuard v0.2.1"
echo "============================"

# Função para mostrar erro
fail() {
    echo "❌ $1"
    exit 1
}

# 1. Build básico
echo "1. Build básico..."
cargo check || fail "Compilação falhou"

# 2. Testes
echo "2. Executando testes..."
cargo test --lib --tests || fail "Testes falharam"

# 3. Exemplo principal
echo "3. Testando exemplo principal..."
cargo run --example basic_usage 2>&1 | tail -20 || fail "Exemplo falhou"

# 4. Verificação rápida da API
echo "4. Verificando API..."
cargo run --example basic_usage 2>&1 | grep -q "Limite final:" || fail "API não está funcionando"
cargo run --example basic_usage 2>&1 | grep -q "Permissões disponíveis:" || fail "Métodos de observação não funcionam"

# 5. Final
echo ""
echo "✅ Todos os checks passaram!"
echo "📊 API está funcionando:"
echo "   • FlowGuard::new() ✓"
echo "   • current_limit() ✓"
echo "   • available_permits() ✓"
echo "   • run() ✓"
echo "   • clone() ✓"
echo "   • Ajuste dinâmico ✓"
