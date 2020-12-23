use crate::commons::custom_fields::CustomFieldsHandler;
use crate::commons::file_utilities::load_yaml;
use crate::commons::structs::{AuthOptions, REST_URI};
use crate::stories::command_args::StoryOps;
use crate::stories::stories_structs::{StoriesHandler, Story, Stories};
use anyhow::{Error, bail};
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

        let story_template: Value = match &options.template_path {
            None => json!(&Story { ..default() }),
            Some(path) => {
                let template = load_yaml(&path).await.unwrap();
                let story: Story = serde_yaml::from_str(&template).unwrap();
                json!(story)
            }
        };

       let yaml_string =
                &load_yaml(&options.file).await.expect("Failed to load stories yaml");

        println!("Yaml String, {}", yaml_string.clone());

        let load: Stories = serde_yaml::from_str::<Stories>(yaml_string).unwrap();

        println!("Load {:?}", load);

        // merge(&mut stories, &story_template);

        // println!("{:?}", stories);
    }

    fn parse_custom_fields(&self, yaml_string: String) {


    }
}
