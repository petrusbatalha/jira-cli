use crate::commons::custom_fields::CustomFieldsHandler;
use crate::commons::file_utilities::load_yaml;
use crate::commons::structs::{AuthOptions, REST_URI};
use crate::stories::command_args::StoryOps;
use crate::stories::stories_structs::{Stories, StoriesHandler, StoryRequest, StoryRequestFields};
use anyhow::{bail, Error};
use json_patch::merge;
use serde_json::json;
use serde_json::Value;
use std::default::default;
use tokio::macros::support::Future;

impl StoriesHandler {
    pub async fn create_story(&self, options: &StoryOps, auth_options: &AuthOptions) {
        let uri = format!("{}{}", &auth_options.host, &REST_URI);

        let custom_fields = CustomFieldsHandler
            .get_or_cache(auth_options, &options.project)
            .await
            .unwrap();

        let mut story_template: StoryRequest = match &options.template_path {
            None => StoryRequest { ..default() },
            Some(path) => {
                let template = load_yaml(&path).await.unwrap();
                let story: StoryRequest = serde_yaml::from_str(&template).unwrap();
                story
            }
        };

        let yaml_string = &load_yaml(&options.file)
            .await
            .expect("Failed to load stories yaml");

        let mut stories_yaml: Stories = serde_yaml::from_str::<Stories>(yaml_string).unwrap();

        for story in stories_yaml.issue_updates.iter_mut() {
            *story = StoryRequestFields::new_or_template(story.clone().fields, story_template.clone());
        }

        println!("{}", json!(stories_yaml))
    }
}
