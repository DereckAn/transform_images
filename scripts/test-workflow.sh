#!/bin/bash
# Script para testear workflows de GitHub Actions

set -e

BRANCH="test-workflow"
CURRENT_BRANCH=$(git branch --show-current)

echo "🧪 Testing GitHub Actions Workflow"
echo "=================================="

# Función para limpiar al salir
cleanup() {
    echo "🧹 Limpiando..."
    git checkout "$CURRENT_BRANCH" 2>/dev/null || true
}
trap cleanup EXIT

# Opción 1: Testear con Act (local)
test_local() {
    echo "📦 Testeando localmente con Act..."
    
    if ! command -v act &> /dev/null; then
        echo "❌ Act no está instalado. Instala con: brew install act"
        exit 1
    fi
    
    echo "Ejecutando workflow localmente..."
    act push --dryrun
}

# Opción 2: Testear en rama dedicada
test_branch() {
    echo "🌿 Testeando en rama $BRANCH..."
    
    # Crear o cambiar a rama de testing
    git checkout -b "$BRANCH" 2>/dev/null || git checkout "$BRANCH"
    
    # Hacer commit de cambios
    git add .
    git commit -m "test: workflow changes" --allow-empty
    
    # Push a GitHub
    git push origin "$BRANCH" --force
    
    echo "✅ Push completado. Ve a GitHub Actions para ver resultados:"
    echo "   https://github.com/DereckAn/transform_images/actions"
    
    # Volver a rama original
    git checkout "$CURRENT_BRANCH"
}

# Opción 3: Simular tag localmente
test_tag() {
    echo "🏷️  Simulando tag v0.6.0-test..."
    
    # Crear event.json para Act
    cat > /tmp/event.json << EOF
{
  "ref": "refs/tags/v0.6.0-test",
  "repository": {
    "full_name": "DereckAn/transform_images"
  }
}
EOF
    
    act push --eventpath /tmp/event.json --dryrun
}

# Menú
echo ""
echo "Selecciona una opción:"
echo "1) Testear localmente con Act (rápido, sin push)"
echo "2) Testear en rama dedicada (real, con push)"
echo "3) Simular tag localmente"
echo "4) Salir"
echo ""
read -p "Opción: " option

case $option in
    1) test_local ;;
    2) test_branch ;;
    3) test_tag ;;
    4) echo "👋 Saliendo..."; exit 0 ;;
    *) echo "❌ Opción inválida"; exit 1 ;;
esac
