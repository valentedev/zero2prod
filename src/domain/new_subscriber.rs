use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::subscriber_name::SubscriberName;

pub struct NewSubscriber {
    //pub email: String,
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
