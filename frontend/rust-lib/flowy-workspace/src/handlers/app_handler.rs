use crate::{
    entities::{
        app::{
            App,
            AppIdentifier,
            CreateAppParams,
            CreateAppRequest,
            QueryAppRequest,
            UpdateAppParams,
            UpdateAppRequest,
        },
        trash::Trash,
    },
    errors::WorkspaceError,
    services::{AppController, TrashCan, ViewController},
};
use lib_dispatch::prelude::{data_result, Data, DataResult, Unit};
use std::{convert::TryInto, sync::Arc};

pub(crate) async fn create_app_handler(
    data: Data<CreateAppRequest>,
    controller: Unit<Arc<AppController>>,
) -> DataResult<App, WorkspaceError> {
    let params: CreateAppParams = data.into_inner().try_into()?;
    let detail = controller.create_app_from_params(params).await?;

    data_result(detail)
}

pub(crate) async fn delete_app_handler(
    data: Data<QueryAppRequest>,
    controller: Unit<Arc<AppController>>,
    trash_can: Unit<Arc<TrashCan>>,
) -> Result<(), WorkspaceError> {
    let params: AppIdentifier = data.into_inner().try_into()?;
    let trash = controller
        .read_app_tables(vec![params.app_id])?
        .into_iter()
        .map(|view_table| view_table.into())
        .collect::<Vec<Trash>>();

    let _ = trash_can.add(trash).await?;
    Ok(())
}

#[tracing::instrument(skip(data, controller))]
pub(crate) async fn update_app_handler(
    data: Data<UpdateAppRequest>,
    controller: Unit<Arc<AppController>>,
) -> Result<(), WorkspaceError> {
    let params: UpdateAppParams = data.into_inner().try_into()?;
    let _ = controller.update_app(params).await?;
    Ok(())
}

#[tracing::instrument(skip(data, app_controller, view_controller))]
pub(crate) async fn read_app_handler(
    data: Data<QueryAppRequest>,
    app_controller: Unit<Arc<AppController>>,
    view_controller: Unit<Arc<ViewController>>,
) -> DataResult<App, WorkspaceError> {
    let params: AppIdentifier = data.into_inner().try_into()?;
    let mut app = app_controller.read_app(params.clone()).await?;
    app.belongings = view_controller.read_views_belong_to(&params.app_id).await?;

    data_result(app)
}
