use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;
use tokio::sync::Mutex;

use crate::types::{
    AppSettings, GitCommitDiff, GitFileDiff, GitHubIssuesResponse, GitHubPullRequestComment,
    GitHubPullRequestDiff, GitHubPullRequestsResponse, GitLogResponse, WorkspaceEntry,
};

const UNSUPPORTED: &str = "Git operations are not available on mobile. Use remote backend mode.";

pub(crate) async fn resolve_repo_root_for_workspace_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<PathBuf, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) fn collect_workspace_diff_core(_repo_root: &Path) -> Result<String, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_git_status_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<Value, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn list_git_roots_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _depth: Option<usize>,
) -> Result<Vec<String>, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_git_diffs_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _app_settings: &Mutex<AppSettings>,
    _workspace_id: String,
) -> Result<Vec<GitFileDiff>, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_git_log_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _limit: Option<usize>,
) -> Result<GitLogResponse, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_git_commit_diff_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _app_settings: &Mutex<AppSettings>,
    _workspace_id: String,
    _sha: String,
) -> Result<Vec<GitCommitDiff>, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_git_remote_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<Option<String>, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn stage_git_file_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _path: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn stage_git_all_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn unstage_git_file_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _path: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn revert_git_file_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _path: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn revert_git_all_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn commit_git_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _message: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn push_git_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn pull_git_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn fetch_git_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn sync_git_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_github_issues_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<GitHubIssuesResponse, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_github_pull_requests_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<GitHubPullRequestsResponse, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_github_pull_request_diff_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _pr_number: u64,
) -> Result<Vec<GitHubPullRequestDiff>, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn get_github_pull_request_comments_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _pr_number: u64,
) -> Result<Vec<GitHubPullRequestComment>, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn list_git_branches_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
) -> Result<Value, String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn checkout_git_branch_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _name: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}

pub(crate) async fn create_git_branch_core(
    _workspaces: &Mutex<HashMap<String, WorkspaceEntry>>,
    _workspace_id: String,
    _name: String,
) -> Result<(), String> {
    Err(UNSUPPORTED.to_string())
}
