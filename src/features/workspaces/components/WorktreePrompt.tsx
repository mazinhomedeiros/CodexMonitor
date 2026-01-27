import { useEffect, useRef } from "react";

type WorktreePromptProps = {
  workspaceName: string;
  branch: string;
  setupScript: string;
  savedSetupScript: string | null;
  scriptError?: string | null;
  error?: string | null;
  onChange: (value: string) => void;
  onSetupScriptChange: (value: string) => void;
  onSaveSetupScript: () => void;
  onCancel: () => void;
  onConfirm: () => void;
  isBusy?: boolean;
  isSavingScript?: boolean;
};

function normalizeScript(value: string | null | undefined): string | null {
  const next = value ?? "";
  return next.trim().length > 0 ? next : null;
}

export function WorktreePrompt({
  workspaceName,
  branch,
  setupScript,
  savedSetupScript,
  scriptError = null,
  error = null,
  onChange,
  onSetupScriptChange,
  onSaveSetupScript,
  onCancel,
  onConfirm,
  isBusy = false,
  isSavingScript = false,
}: WorktreePromptProps) {
  const inputRef = useRef<HTMLInputElement | null>(null);
  const scriptChanged = normalizeScript(setupScript) !== savedSetupScript;

  useEffect(() => {
    inputRef.current?.focus();
    inputRef.current?.select();
  }, []);

  return (
    <div className="worktree-modal" role="dialog" aria-modal="true">
      <div
        className="worktree-modal-backdrop"
        onClick={() => {
          if (!isBusy) {
            onCancel();
          }
        }}
      />
      <div className="worktree-modal-card">
        <div className="worktree-modal-title">New worktree agent</div>
        <div className="worktree-modal-subtitle">
          Create a worktree under "{workspaceName}".
        </div>
        <label className="worktree-modal-label" htmlFor="worktree-branch">
          Branch name
        </label>
        <input
          id="worktree-branch"
          ref={inputRef}
          className="worktree-modal-input"
          value={branch}
          onChange={(event) => onChange(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Escape") {
              event.preventDefault();
              if (!isBusy) {
                onCancel();
              }
            }
            if (event.key === "Enter" && !isBusy) {
              event.preventDefault();
              onConfirm();
            }
          }}
        />
        <div className="worktree-modal-divider" />
        <div className="worktree-modal-section-title">Worktree setup script</div>
        <div className="worktree-modal-hint">
          Runs once in a dedicated terminal after each new worktree is created.
        </div>
        <textarea
          id="worktree-setup-script"
          className="worktree-modal-textarea"
          value={setupScript}
          onChange={(event) => onSetupScriptChange(event.target.value)}
          placeholder="pnpm install"
          rows={4}
          disabled={isBusy || isSavingScript}
        />
        <div className="worktree-modal-inline-actions">
          <button
            className="ghost worktree-modal-button"
            onClick={onSaveSetupScript}
            type="button"
            disabled={isBusy || isSavingScript || !scriptChanged}
          >
            {isSavingScript ? "Saving…" : scriptChanged ? "Save script" : "Saved"}
          </button>
        </div>
        {scriptError && <div className="worktree-modal-error">{scriptError}</div>}
        {error && <div className="worktree-modal-error">{error}</div>}
        <div className="worktree-modal-actions">
          <button
            className="ghost worktree-modal-button"
            onClick={onCancel}
            type="button"
            disabled={isBusy}
          >
            Cancel
          </button>
          <button
            className="primary worktree-modal-button"
            onClick={onConfirm}
            type="button"
            disabled={isBusy || branch.trim().length === 0}
          >
            Create
          </button>
        </div>
      </div>
    </div>
  );
}
