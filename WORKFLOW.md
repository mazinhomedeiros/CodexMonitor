# CodexMonitor — Workflow de Desenvolvimento e Deploy

## 1. Sincronizando com o Repositório Original (Upstream)

O projeto original é `Dimillian/CodexMonitor` (MIT license). Você pode fazer pull das atualizações, mas com cuidado.

### Riscos de `git pull` direto

| Risco | Mitigação |
|-------|-----------|
| Configs de macOS sobrescreverem suas configs Windows | Use `.gitattributes` e revise conflitos |
| `Cargo.toml` perder suas alterações Android | Faça backup antes, restaure após merge |
| `.cargo/config.toml` ser sobrescrito | Adicione ao `.gitignore` local ou use `git update-index --skip-worktree` |

### Workflow seguro para atualização

```bash
# 1. Adicione o upstream (só uma vez)
git remote add upstream https://github.com/Dimillian/CodexMonitor.git

# 2. Salve suas alterações locais
git stash push -m "minhas-alteracoes-android"

# 3. Fetch do upstream
git fetch upstream

# 4. Veja o que mudou (sem aplicar)
git log HEAD..upstream/main --oneline

# 5. Merge ou rebase com cuidado
git merge upstream/main  # ou: git rebase upstream/main

# 6. Restaure suas alterações
git stash pop
```

### Protegendo arquivos locais

```bash
# Arquivos que você NÃO quer que sejam sobrescritos:
git update-index --skip-worktree src-tauri/.cargo/config.toml
git update-index --skip-worktree src-tauri/tauri.windows.conf.json

# Para reverter:
git update-index --no-skip-worktree src-tauri/.cargo/config.toml
```

## 2. Recompilação após Pull

### Quando você PRECISA recompilar:

- Mudanças em `src-tauri/src/*.rs` (código Rust)
- Mudanças em `Cargo.toml` (dependências)
- Mudanças em `src-tauri/tauri.conf.json` (configuração Tauri)

### Quando você NÃO precisa recompilar:

- Mudanças apenas no frontend (`src/*.ts`, `src/*.tsx`)
- Mudanças em documentação
- Mudanças em assets

### Comando rápido para recompilar só o daemon:

```powershell
# No src-tauri/
cargo build --bin codex_monitor_daemon --release
```

## 3. Publicando no Seu GitHub

### O que SUBIR (commit/push):

```
✅ src/                    (código fonte frontend)
✅ src-tauri/src/          (código fonte Rust)
✅ src-tauri/Cargo.toml    (com suas alterações Android)
✅ src-tauri/tauri.android.conf.json
✅ src-tauri/tauri.windows.conf.json
✅ .cargo/config.toml      (com paths Windows)
✅ DAEMON_DEPLOY.md        (documentação)
✅ README.md               (atualizado com Android)
✅ package.json            (scripts npm)
```

### O que NÃO SUBIR (adicionar ao `.gitignore`):

```
❌ src-tauri/target/           (binários compilados)
❌ node_modules/             (dependências npm)
❌ *.exe, *.dll, *.so        (binários)
❌ src-tauri/gen/android/app/build/  (build Android)
```

### `.gitignore` recomendado (adicione se não existir):

```gitignore
# Build artifacts
src-tauri/target/
node_modules/
dist/
*.exe
*.dll
*.so
*.dylib

# Android build
src-tauri/gen/android/app/build/
src-tauri/gen/android/.gradle/

# IDE
.vscode/
.idea/
*.iml

# OS
.DS_Store
Thumbs.db

# Local configs (opcional)
# src-tauri/.cargo/config.toml  # se contiver paths locais
```

### Criando seu repositório

**Opção A: Fork no GitHub (recomendado)**
1. Vá em https://github.com/Dimillian/CodexMonitor
2. Clique **Fork** (canto superior direito)
3. Clone seu fork: `git clone https://github.com/SEU_USER/CodexMonitor.git`
4. Adicione upstream: `git remote add upstream https://github.com/Dimillian/CodexMonitor.git`
5. Aplique suas alterações locais no fork
6. Push: `git push origin main`

**Opção B: Novo repositório (se quiser histórico limpo)**
1. Crie novo repo no GitHub (vazio)
2. No seu projeto local:
   ```bash
   git remote rename origin upstream
   git remote add origin https://github.com/SEU_USER/MeuCodexMonitor.git
   git push -u origin main
   ```

## 4. Compartilhando os Binários

Não commite os `.exe`. Em vez disso:

### Release no GitHub (recomendado)

```bash
# 1. Crie a release via GitHub web
# 2. Faça upload manual dos arquivos:
#    - codex_monitor_daemon.exe
#    - codex-monitor.exe (se quiser)
#    - app-universal-debug.apk (Android)
```

### Ou: armazenamento externo

- Google Drive / Dropbox / OneDrive
- Compartilhe link com os `.exe` e instruções do `DAEMON_DEPLOY.md`

## 5. Checklist antes de cada `git pull` do upstream

- [ ] Backup dos arquivos modificados: `.cargo/config.toml`, `Cargo.toml`
- [ ] `git stash` das alterações não commitadas
- [ ] Review do diff: `git log HEAD..upstream/main --stat`
- [ ] Verificar conflitos em arquivos de configuração
- [ ] Testar build após merge: `cargo check`
- [ ] Recompilar daemon se necessário

## 6. Resolução de Conflitos Comuns

### Conflito em `Cargo.toml`

```toml
# Se vier assim (upstream - macOS only):
[dependencies]
git2 = "0.20.3"

# Mantenha assim (sua versão - Android compatible):
[target."cfg(not(any(target_os = \"android\", target_os = \"ios\")))".dependencies]
git2 = { version = "0.20.3", features = ["vendored-openssl", "vendored-libgit2"] }
```

### Conflito em `.cargo/config.toml`

```toml
# Se vier assim (upstream - macOS paths):
[target.aarch64-linux-android]
linker = "/Users/.../ndk/..."

# Substitua por seus paths Windows:
[target.aarch64-linux-android]
linker = "C:/Users/mazin/Android/Sdk/ndk/..."
```

---

**Dica**: Configure seus arquivos locais uma vez, commit no SEU repo, e use `git merge` em vez de `git pull` para ter mais controle sobre o que entra.
