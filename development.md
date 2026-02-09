Plano Detalhado: Suporte Android para CodexMonitor
Fase 0 — Pré-requisitos (ambiente de desenvolvimento)
Antes de tocar no código, o desenvolvedor precisa instalar:

Android Studio (inclui SDK Manager e AVD Manager)
Android SDK (API level 24+ mínimo para Tauri 2)
Android NDK (versão recomendada pelo Tauri — tipicamente r25c ou r26)
JDK 17+
Rust Android targets:
bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
Variáveis de ambiente:
bash
ANDROID_HOME=/path/to/android/sdk
NDK_HOME=$ANDROID_HOME/ndk/<version>
JAVA_HOME=/path/to/jdk
Fase 1 — Gerar o projeto Android
Passo 1.1: tauri android init
bash
npx tauri android init
Isso cria src-tauri/gen/android/ (hoje só existe gen/apple/). Gera:

app/ — projeto Gradle com MainActivity.kt
build.gradle.kts — configuração de build
gradle/ — wrapper
Passo 1.2: Verificar ícones
Os ícones Android já existem em src-tauri/icons/android/ com todas as densidades (mipmap-hdpi até mipmap-xxxhdpi). Confirmar que estão populados (alguns diretórios mostram 0 items — podem precisar de regeneração):

bash
npx tauri icon src-tauri/icons/icon.png
Fase 2 — Ajustes no Backend Rust
Passo 2.1: build.rs — adicionar link flags para Android (se necessário)
Arquivo: src-tauri/build.rs

Hoje só trata iOS:

rust
if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("ios") {
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=iconv");
}
Pode precisar de flags equivalentes para Android se git2 (vendored OpenSSL/libgit2) exigir. Testar primeiro — se compilar sem mudanças, não mexer.

Passo 2.2: tauri-plugin-liquid-glass — verificar compatibilidade Android
Arquivo: src-tauri/Cargo.toml linha 26

toml
tauri-plugin-liquid-glass = "0.1"
Este plugin é para efeitos visuais Apple (visionOS/macOS). Se não compilar para Android, precisa ser movido para:

toml
[target."cfg(any(target_os = \"macos\", target_os = \"ios\"))".dependencies]
tauri-plugin-liquid-glass = "0.1"
E o .plugin(tauri_plugin_liquid_glass::init()) em src-tauri/src/lib.rs:100 precisa de cfg-gate:

rust
#[cfg(any(target_os = "macos", target_os = "ios"))]
let builder = builder.plugin(tauri_plugin_liquid_glass::init());
Passo 2.3: fix-path-env — verificar compatibilidade Android
Arquivo: src-tauri/Cargo.toml linha 39

toml
fix-path-env = { git = "https://github.com/tauri-apps/fix-path-env-rs" }
Este crate corrige $PATH em apps GUI no macOS. Provavelmente é no-op no Android, mas se não compilar, precisa de cfg-gate similar.

Passo 2.4: window.rs — adicionar configuração Android webview (opcional)
Arquivo: src-tauri/src/window.rs

Hoje existe configure_ios_webview_edge_to_edge (linhas 79-116) para iOS. Pode ser necessário um equivalente Android para edge-to-edge display, mas o Tauri 2 já lida com isso razoavelmente no Android. Pode ser adicionado depois se necessário.

Passo 2.5: lib.rs — setup block para Android (opcional)
Arquivo: src-tauri/src/lib.rs:82-87

Hoje existe um bloco #[cfg(target_os = "ios")] no setup. Se precisar de configuração Android-specific, adicionar bloco equivalente:

rust
#[cfg(target_os = "android")]
{
    // Android-specific setup if needed
}
Nota: Provavelmente não é necessário na primeira iteração.

Fase 3 — Verificação de Compilação Cruzada de Crates
Estas crates precisam compilar para aarch64-linux-android. Risco por crate:

Crate	Risco	Notas
git2 (vendored)	Médio	OpenSSL vendored geralmente funciona com NDK, mas pode precisar de OPENSSL_DIR ou ajustes no toolchain
reqwest (rustls-tls)	Baixo	rustls é pure-Rust, compila em qualquer target
tokio	Zero	Funciona em Android
serde/serde_json	Zero	Pure Rust
tokio-tungstenite	Baixo	WebSocket pure-Rust com rustls
chrono	Zero	Pure Rust
libc	Zero	Suporta Android nativamente
base64, uuid, toml, shell-words, ignore	Zero	Pure Rust
Ação: Rodar cargo check --target aarch64-linux-android no src-tauri/ para identificar problemas reais antes de qualquer outra mudança.

Fase 4 — Configuração Tauri
Passo 4.1: tauri.conf.json — adicionar seção Android bundle
Arquivo: src-tauri/tauri.conf.json

Adicionar dentro de bundle:

json
"android": {
  "minSdkVersion": 24
}
Passo 4.2: Criar tauri.android.conf.json (opcional, como o Windows)
Seguindo o padrão de tauri.windows.conf.json, criar um override config para Android:

Novo arquivo: src-tauri/tauri.android.conf.json

json
{
  "app": {
    "windows": [
      {
        "title": "Codex Monitor",
        "width": 1200,
        "height": 700,
        "minWidth": 360,
        "minHeight": 600,
        "dragDropEnabled": false,
        "titleBarStyle": "Visible",
        "hiddenTitle": false,
        "transparent": false,
        "devtools": true
      }
    ]
  }
}
Fase 5 — Frontend (mínimo necessário)
Passo 5.1: Verificar liquid-glass-api no Android
Arquivo: package.json linha 56

json
"tauri-plugin-liquid-glass-api": "^0.1.6"
Se o plugin backend for cfg-gated, o JS API precisa de graceful fallback. Verificar se já tem.

Passo 5.2: main.tsx — Sentry platform tag
Arquivo: src/main.tsx:20-21

typescript
Sentry.metrics.count("app_open", 1, {
  attributes: {
    env: import.meta.env.MODE,
    platform: "macos",  // ← hardcoded!
  },
});
Mudar para detecção dinâmica (melhoria geral, não bloqueante).

Passo 5.3: Nenhuma mudança necessária no mobile flow
O frontend já está pronto:

isMobilePlatform() detecta "android" no userAgent
useLayoutMode() força phone layout em mobile
useMobileServerSetup() é platform-agnostic
MobileServerSetupWizard é platform-agnostic
Viewport height sync e gesture prevention já tratam Android
Fase 6 — Scripts de Build
Passo 6.1: Script de build para emulador Android
Novo arquivo: scripts/build_run_android.sh

Equivalente ao build_run_ios.sh, com:

npx tauri android dev para dev mode
npx tauri android build para release
Opção de target device/emulador
Passo 6.2: npm scripts
Arquivo: package.json

Adicionar:

json
"tauri:dev:android": "tauri android dev",
"tauri:build:android": "tauri android build"
Fase 7 — README
Passo 7.1: Adicionar seção Android Support
Arquivo: README.md

Adicionar seção equivalente à "iOS Support (WIP)" com:

Status atual
Pré-requisitos (Android Studio, SDK, NDK, JDK, Rust targets)
Como rodar no emulador
Como rodar em device USB
Setup com Tailscale/Orbit (idêntico ao iOS)
Resumo de Arquivos Afetados
Arquivo	Tipo de Mudança
src-tauri/Cargo.toml	Possível cfg-gate de liquid-glass e fix-path-env
src-tauri/build.rs	Possível adição de link flags Android
src-tauri/src/lib.rs	Possível cfg-gate do plugin liquid-glass init
src-tauri/tauri.conf.json	Adicionar bundle.android
src-tauri/tauri.android.conf.json	Novo — override config Android
scripts/build_run_android.sh	Novo — script de build
package.json	Adicionar npm scripts Android
README.md	Adicionar seção Android
src/main.tsx	Fix Sentry platform tag (opcional)
Ordem de Execução Recomendada
Instalar pré-requisitos (SDK, NDK, targets)
npx tauri android init
cargo check --target aarch64-linux-android no src-tauri/ → identificar erros de compilação
Resolver erros (cfg-gates, link flags)
npx tauri android dev → testar no emulador
Testar remote backend flow (Tailscale/Orbit)
Criar scripts e documentação
Polish (edge-to-edge, status bar, etc.)
O passo 3 é o mais crítico — ele revela exatamente o que precisa de ajuste. Todo o resto é configuração e documentação.

A codebase está excepcionalmente bem preparada para Android. O padrão cfg!(any(target_os = "ios", target_os = "android")) já está em todos os pontos críticos, os stubs mobile para terminal e dictation já existem, e o frontend mobile flow é totalmente platform-agnostic. O trabalho restante é majoritariamente de configuração e toolchain, não de código.