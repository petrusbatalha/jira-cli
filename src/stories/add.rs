use crate::stories::stories::StoriesHandler;
use crate::stories::command_args::StoryOps;
use crate::commons::structs::AuthOptions;

impl StoriesHandler {
    pub async fn create_story(&self, _options: &StoryOps, _auth_options: &AuthOptions) {
        println!("{:?}", "add stories");
    }
}
