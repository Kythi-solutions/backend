use sea_orm::prelude::async_trait::async_trait;
use sea_orm::sea_query::IntoValueTuple;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, FromQueryResult, InsertResult, IntoActiveModel, Iterable, ModelTrait, PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter};

pub mod user;

// E: Entity Trait
// M: Model Trait
// A: Active Model Trait
#[async_trait]
pub trait Repository<
    E: EntityTrait<Model = M>,
    M: ModelTrait + FromQueryResult + Sync + IntoActiveModel<A>,
    A: ActiveModelTrait + ActiveModelBehavior + std::marker::Send> {
    fn get_db(&self) -> std::sync::Arc<DatabaseConnection>;

    async fn find_by_id(&self, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Result<Option<M>, DbErr> {
        let db = self.get_db();

        E::find_by_id(id).one(db.as_ref()).await
    }

}

#[macro_export]
macro_rules! repository {
    ($repo: ty, $e:ty, $m:ty, $a: ty) => {
        use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, InsertResult, ModelTrait, PrimaryKeyTrait, QueryFilter, ColumnTrait, ColumnDef, PrimaryKeyToColumn, DbErr, DeleteMany, Iterable, UpdateResult};
        use sea_orm::prelude::async_trait::async_trait;
        impl Repository<$e, $m, $a> for $repo {
             fn get_db(&self) -> std::sync::Arc<DatabaseConnection> {
                self.db.clone()
             }
        }
    }
}