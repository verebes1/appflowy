mod server_api;
mod server_api_mock;
pub use server_api::*;
// TODO: exclude mock files in production
pub use server_api_mock::*;

use crate::{
    entities::{
        app::{CreateAppParams, DeleteAppParams, QueryAppParams, UpdateAppParams},
        view::{CreateViewParams, DeleteViewParams, QueryViewParams, UpdateViewParams},
        workspace::{CreateWorkspaceParams, DeleteWorkspaceParams, QueryWorkspaceParams, RepeatedWorkspace, UpdateWorkspaceParams},
    },
    errors::WorkspaceError,
};
use flowy_infra::future::ResultFuture;
use std::sync::Arc;

pub(crate) type Server = Arc<dyn WorkspaceServerAPI + Send + Sync>;

pub trait WorkspaceServerAPI {
    // Workspace
    fn create_workspace(&self, token: &str, params: CreateWorkspaceParams) -> ResultFuture<(), WorkspaceError>;

    fn read_workspace(&self, token: &str, params: QueryWorkspaceParams) -> ResultFuture<RepeatedWorkspace, WorkspaceError>;

    fn update_workspace(&self, token: &str, params: UpdateWorkspaceParams) -> ResultFuture<(), WorkspaceError>;

    fn delete_workspace(&self, token: &str, params: DeleteWorkspaceParams) -> ResultFuture<(), WorkspaceError>;

    // View
    fn create_view(&self, token: &str, params: CreateViewParams) -> ResultFuture<(), WorkspaceError>;

    fn read_view(&self, token: &str, params: QueryViewParams) -> ResultFuture<(), WorkspaceError>;

    fn delete_view(&self, token: &str, params: DeleteViewParams) -> ResultFuture<(), WorkspaceError>;

    fn update_view(&self, token: &str, params: UpdateViewParams) -> ResultFuture<(), WorkspaceError>;

    // App
    fn create_app(&self, token: &str, params: CreateAppParams) -> ResultFuture<(), WorkspaceError>;
    fn read_app(&self, token: &str, params: QueryAppParams) -> ResultFuture<(), WorkspaceError>;
    fn update_app(&self, token: &str, params: UpdateAppParams) -> ResultFuture<(), WorkspaceError>;
    fn delete_app(&self, token: &str, params: DeleteAppParams) -> ResultFuture<(), WorkspaceError>;
}

pub(crate) fn construct_workspace_server() -> Arc<dyn WorkspaceServerAPI + Send + Sync> {
    if cfg!(feature = "http_server") {
        Arc::new(WorkspaceServer {})
    } else {
        Arc::new(WorkspaceServerMock {})
    }
}