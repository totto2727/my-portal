use sea_orm::ActiveModelTrait;

pub trait IntoActiveModel<A>
where
    A: ActiveModelTrait,
{
    fn into_active_model(self) -> A;
}
