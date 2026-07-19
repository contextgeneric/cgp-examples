use cgp::prelude::*;

use crate::types::*;

#[cgp_component(UserManager)]
pub trait CanManageUser {
    fn create_user(&self, username: &str, email: &Email) -> Result<User, Error>;

    fn get_user(&self, user_id: &UserId) -> Result<User, Error>;

    fn update_user_data(&self, user_id: &UserId, user_data: &UserData) -> Result<(), Error>;
}

#[cgp_component(PostManager)]
pub trait CanManagePost {
    fn create_post(&self, title: &str, content: &str) -> Result<Post, Error>;

    fn get_post(&self, post_id: &PostId) -> Result<Post, Error>;

    fn update_post(&self, post_id: &PostId, content: &str) -> Result<(), Error>;

    fn delete_post(&self, post_id: &PostId) -> Result<(), Error>;
}

#[cgp_component(UsernameCensor)]
pub trait CanCensorUsername {
    fn username_is_censored(&self, username: &str) -> Probability;
}

#[cgp_component(SpamMessageDetector)]
pub trait CanDetectSpamMessage {
    fn message_is_spam(&self, message: &str) -> Probability;
}

#[cgp_impl(new PostgresUserManager)]
#[uses(CanCensorUsername)]
impl UserManager {
    fn create_user(
        &self,
        #[implicit] database: &PostgresDb,
        username: &str,
        email: &Email,
    ) -> Result<User, Error> {
        if self.username_is_censored(username) > Probability::new(0.8) {
            return Err(Error::InvalidUsername);
        }

        todo!()
    }

    fn get_user(&self, #[implicit] database: &PostgresDb, user_id: &UserId) -> Result<User, Error> {
        todo!()
    }

    fn update_user_data(
        &self,
        #[implicit] database: &PostgresDb,
        user_id: &UserId,
        user_data: &UserData,
    ) -> Result<(), Error> {
        todo!()
    }
}

#[cgp_impl(new PostgresPostManager)]
#[uses(CanDetectSpamMessage)]
impl PostManager {
    fn create_post(
        &self,
        #[implicit] database: &PostgresDb,
        title: &str,
        content: &str,
    ) -> Result<Post, Error> {
        if self.message_is_spam(content) > Probability::new(0.8) {
            return Err(Error::InvalidMessage);
        }

        todo!()
    }

    fn get_post(&self, #[implicit] database: &PostgresDb, post_id: &PostId) -> Result<Post, Error> {
        todo!()
    }

    fn update_post(
        &self,
        #[implicit] database: &PostgresDb,
        post_id: &PostId,
        content: &str,
    ) -> Result<(), Error> {
        todo!()
    }

    fn delete_post(
        &self,
        #[implicit] database: &PostgresDb,
        post_id: &PostId,
    ) -> Result<(), Error> {
        todo!()
    }
}

#[cgp_impl(new DummyUserCensor)]
impl UsernameCensor {
    fn username_is_censored(&self, username: &str) -> Probability {
        todo!()
    }
}

#[cgp_impl(new DummySpamMessageDetector)]
impl SpamMessageDetector {
    fn message_is_spam(&self, message: &str) -> Probability {
        todo!()
    }
}

#[derive(HasField)]
pub struct ProductionApp {
    pub database: PostgresDb,
}

delegate_and_check_components! {
    ProductionApp {
        UserManagerComponent: PostgresUserManager,
        PostManagerComponent: PostgresPostManager,
        UsernameCensorComponent: DummyUserCensor,
        SpamMessageDetectorComponent: DummySpamMessageDetector,
    }
}
