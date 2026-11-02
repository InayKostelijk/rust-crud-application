use std::sync::Arc;
use sqlx::MySqlPool;
use crate::v1::repository::OrderedListRepository;
use crate::v1::repository_impls::OrderedListRepositoryImpls;

#[derive(Clone)]
pub struct State(Arc<StateInner>);
pub struct StateInner {
    pub ordered_list_repository: Arc<dyn OrderedListRepository>,
}

// waarom dit?
impl std::ops::Deref for State {
    type Target = StateInner;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl State {
    pub fn new(pool: MySqlPool)

    -> Self{
        let ordered_list_repository = OrderedListRepositoryImpls::new(pool);



        Self(Arc::new(StateInner {


            ordered_list_repository: Arc::new(ordered_list_repository) 
        }))
    }
}

