use sea_orm::DeleteResult;

pub mod user;
mod file;
mod invite;

// E: Entity
// K: primary key type
pub trait Repository<E, K> {
    fn by_id(&self, id: K) -> impl Future<Output = Result<Option<E>, sea_orm::DbErr>> + Send;
    fn create(&self, model: E) -> impl Future<Output = Result<E, sea_orm::DbErr>> + Send;
    fn update(&self, model: E) -> impl Future<Output = Result<E, sea_orm::DbErr>> + Send;
    fn delete(&self, model: E) -> impl Future<Output = Result<DeleteResult, sea_orm::DbErr>> + Send;
    fn delete_by_id(&self, id: K) -> impl Future<Output = Result<DeleteResult, sea_orm::DbErr>> + Send;
}

#[macro_export]
macro_rules! repository_macro {
    ($model:path, $repo:ty, $id_ty:ty) => {
        use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, DeleteResult};
        use $model as Model;
        impl Repository<Model::Model, $id_ty> for $repo {
             async fn by_id(&self, id: $id_ty) -> Result<Option<Model::Model>, sea_orm::DbErr> {
                Model::Entity::find_by_id(id).one(&self.db).await
             }

            async fn create(&self, model: Model::Model) -> Result<Model::Model, sea_orm::DbErr> {
                // Into ActiveModel
                let active_model: Model::ActiveModel = model.into();

                active_model.insert(&self.db).await
            }

            async fn update(&self, model: Model::Model) -> Result<Model::Model, sea_orm::DbErr> {
                let active_model: Model::ActiveModel = model.into();

                active_model.update(&self.db).await
            }

            async fn delete(&self, model: Model::Model) -> Result<DeleteResult, sea_orm::DbErr> {
                let active_model: Model::ActiveModel = model.into();

                active_model.delete(&self.db).await
            }

            async fn delete_by_id(&self, id: $id_ty) -> Result<DeleteResult, sea_orm::DbErr> {
                Model::Entity::delete_by_id(id).exec(&self.db).await
            }
        }
    }
}