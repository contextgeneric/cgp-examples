use cgp::prelude::*;

use crate::types::*;

#[cgp_component(UserCreator)]
#[prefix(@app.core.user in DefaultNamespace)]
pub trait CanCreateUser {
    fn create_user(&self, username: &str, email: &Email) -> Result<User, Error>;
}

#[cgp_component(UserGetter)]
#[prefix(@app.core.user in DefaultNamespace)]
pub trait CanGetUser {
    fn get_user(&self, user_id: &UserId) -> Result<User, Error>;
}

#[cgp_component(UserUpdater)]
#[prefix(@app.core.user in DefaultNamespace)]
pub trait CanUpdateUser {
    fn update_user_data(&self, user_id: &UserId, user_data: &UserData) -> Result<(), Error>;
}

#[cgp_component(PostCreator)]
#[prefix(@app.core.post in DefaultNamespace)]
pub trait CanCreatePost {
    fn create_post(&self, title: &str, content: &str) -> Result<Post, Error>;
}

#[cgp_component(PostGetter)]
#[prefix(@app.core.post in DefaultNamespace)]
pub trait CanGetPost {
    fn get_post(&self, post_id: &PostId) -> Result<Post, Error>;
}

#[cgp_component(PostUpdater)]
#[prefix(@app.core.post in DefaultNamespace)]
pub trait CanUpdatePost {
    fn update_post(&self, post_id: &PostId, content: &str) -> Result<(), Error>;
}

#[cgp_component(PostDeleter)]
#[prefix(@app.core.post in DefaultNamespace)]
pub trait CanDeletePost {
    fn delete_post(&self, post_id: &PostId) -> Result<(), Error>;
}

#[cgp_component(UsernameCensor)]
#[prefix(@app.extra.content_filter in DefaultNamespace)]
pub trait CanCensorUsername {
    fn username_is_censored(&self, username: &str) -> Probability;
}

#[cgp_component(SpamMessageDetector)]
#[prefix(@app.extra.content_filter in DefaultNamespace)]
pub trait CanDetectSpamMessage {
    fn message_is_spam(&self, message: &str) -> Probability;
}

#[cgp_impl(new FilterCensoredUsername<InnerCreator>)]
#[uses(CanCensorUsername)]
#[use_provider(InnerCreator: UserCreator)]
impl<InnerCreator> UserCreator {
    fn create_user(&self, username: &str, email: &Email) -> Result<User, Error> {
        if self.username_is_censored(username) > Probability::new(0.8) {
            return Err(Error::InvalidUsername);
        }

        InnerCreator::create_user(self, username, email)
    }
}

#[cgp_impl(new CreateUserWithPostgres)]
impl UserCreator {
    fn create_user(
        &self,
        #[implicit] database: &PostgresDb,
        username: &str,
        email: &Email,
    ) -> Result<User, Error> {
        todo!()
    }
}

#[cgp_impl(new GetUserWithPostgres)]
impl UserGetter {
    fn get_user(&self, #[implicit] database: &PostgresDb, user_id: &UserId) -> Result<User, Error> {
        todo!()
    }
}

#[cgp_impl(new UpdateUserWithPostgres)]
impl UserUpdater {
    fn update_user_data(
        &self,
        #[implicit] database: &PostgresDb,
        user_id: &UserId,
        user_data: &UserData,
    ) -> Result<(), Error> {
        todo!()
    }
}

#[cgp_impl(new FilterSpamMessage<InnerCreator>)]
#[uses(CanDetectSpamMessage)]
#[use_provider(InnerCreator: PostCreator)]
impl<InnerCreator> PostCreator {
    fn create_post(&self, title: &str, content: &str) -> Result<Post, Error> {
        if self.message_is_spam(content) > Probability::new(0.8) {
            return Err(Error::InvalidMessage);
        }

        InnerCreator::create_post(self, title, content)
    }
}

#[cgp_impl(new CreatePostWithPostgres)]
#[uses(CanDetectSpamMessage)]
impl PostCreator {
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
}

#[cgp_impl(new GetPostWithPostgres)]
impl PostGetter {
    fn get_post(&self, #[implicit] database: &PostgresDb, post_id: &PostId) -> Result<Post, Error> {
        todo!()
    }
}

#[cgp_impl(new UpdatePostWithPostgres)]
impl PostUpdater {
    fn update_post(
        &self,
        #[implicit] database: &PostgresDb,
        post_id: &PostId,
        content: &str,
    ) -> Result<(), Error> {
        todo!()
    }
}

#[cgp_impl(new DeletePostWithPostgres)]
impl PostDeleter {
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

#[cgp_impl(new AiUserCensor)]
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

#[cgp_impl(new AiSpamMessageDetector)]
impl SpamMessageDetector {
    fn message_is_spam(&self, message: &str) -> Probability {
        todo!()
    }
}

#[derive(HasField)]
pub struct ProductionApp {
    pub database: PostgresDb,
}

// delegate_components! {
//     ProductionApp {
//         UserCreatorComponent:
//             FilterCensoredUsername<CreateUserWithPostgres>,
//         UserGetterComponent:
//             GetUserWithPostgres,
//         UserUpdaterComponent:
//             UpdateUserWithPostgres,
//         PostCreatorComponent:
//             FilterSpamMessage<CreatePostWithPostgres>,
//         PostGetterComponent:
//             GetPostWithPostgres,
//         PostUpdaterComponent:
//             UpdatePostWithPostgres,
//         PostDeleterComponent:
//             DeletePostWithPostgres,
//         UsernameCensorComponent:
//             AiUserCensor,
//         SpamMessageDetectorComponent:
//             AiSpamMessageDetector,
//     }
// }

delegate_components! {
    new PostgresUserComponents {
        UserCreatorComponent:
            FilterCensoredUsername<CreateUserWithPostgres>,
        UserGetterComponent:
            GetUserWithPostgres,
        UserUpdaterComponent:
            UpdateUserWithPostgres,
    }
}

delegate_components! {
    new PostgresPostComponents {
        PostCreatorComponent:
            FilterSpamMessage<CreatePostWithPostgres>,
        PostGetterComponent:
            GetPostWithPostgres,
        PostUpdaterComponent:
            UpdatePostWithPostgres,
        PostDeleterComponent:
            DeletePostWithPostgres,
    }
}

delegate_components! {
    new ContentFilterComponents {
        UsernameCensorComponent:
            AiUserCensor,
        SpamMessageDetectorComponent:
            AiSpamMessageDetector,
    }
}

delegate_components! {
    ProductionApp {
        namespace DefaultNamespace;

        @app.core: PostgresCoreComponents,
        @app.extra.content_filter: ContentFilterComponents,
    }
}

check_components! {
    ProductionApp {
        UserCreatorComponent,
    }
}

delegate_components! {
    new PostgresCoreComponents {
        namespace DefaultNamespace;

        @app.core.user: PostgresUserComponents,
        @app.core.post: PostgresPostComponents,
    }
}

delegate_components! {
    new ProductionExtraComponents {
        namespace DefaultNamespace;

        @app.extra.content_filter: ContentFilterComponents,
    }
}
