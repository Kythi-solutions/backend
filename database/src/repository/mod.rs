use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, InsertResult, ModelTrait, PrimaryKeyTrait, QueryFilter, ColumnTrait, ColumnDef, PrimaryKeyToColumn, DbErr, DeleteMany, Iterable, UpdateResult};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::sea_query::ColumnRef::Column;
use sea_orm::sea_query::ExprTrait;
use sea_orm::TryGetError::DbErr;

pub mod user;
mod file;
mod invite;

// E: Entity
// K: primary key type
#[async_trait]
pub trait Repository<E: EntityTrait, M: ModelTrait, A: ActiveModelTrait> {

    fn get_db(&self) -> DatabaseConnection;

    async fn find_by_id(&self, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Result<Option<E>, sea_orm::DbErr> {
        let db = self.get_db();

        E::find_by_id(id).one(&db).await
    }
    async fn create(&self, model: <E::ActiveModel as ActiveModelTrait>::Entity) -> Result<InsertResult<A>, sea_orm::DbErr> {
        let db = self.get_db();

        E::insert(model).exec(&db).await
    }
    async fn update_by_id(&self, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType, active_model: A) -> Result<UpdateResult, DbErr>
    {
        let db = self.get_db();
        let mut update = E::update_many().set(active_model);
        let mut keys = E::PrimaryKey::iter();
        for v in id.into().into_value_tuple() {
            if let Some(key) = keys.next() {
                let col = key.into_column();
                update = update.filter(col.eq(v));
            } else {
                return DbErr("arity mismatch");
            }
        }
        if keys.next().is_some() {
            return DbErr("arity mismatch");
        }
        update.exec(&db).await
    }
    async fn delete_by_id(&self, id: <E::PrimaryKey as ColumnTrait>::EntityName) -> Result<DeleteResult, DbErr> {
        let db = self.get_db();

        E::delete_by_id(id).exec(&db).await
    }
}

#[macro_export]
macro_rules! repository_macro {
    ($model:path, $repo:ty, $id_ty:ty) => {
        use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, DeleteResult};
        use $model as Model;
        impl Repository<Model::Model, $id_ty> for $repo {
             fn get_db(&self) -> DatabaseConnection {
                self.db.clone()
             }
        }
    }
}