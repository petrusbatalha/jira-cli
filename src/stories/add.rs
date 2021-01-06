use crate::commons::file_utilities::load_yaml;
use crate::commons::req_builder::build_post_req;
use crate::commons::structs::{AuthOptions, REST_URI};
use crate::stories::command_args::StoryOps;
use crate::stories::stories_structs::{Stories, StoriesHandler, StoryRequest, StoryRequestFields};
use serde_json::json;
use serde_json::Value;
use std::default::default;
use url::Url;

impl StoriesHandler {
    pub async fn create_story(&self, options: &StoryOps, auth_options: &AuthOptions) {
        let uri = Url::parse(&format!("{}{}/issue/bulk", &auth_options.host, &REST_URI)).unwrap();
        println!("Uri {}", uri);
        let story_template: StoryRequest = match &options.template_path {
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
            *story =
                StoryRequestFields::new_or_template(story.clone().fields, story_template.clone());
        }

        let req = build_post_req(uri, auth_options)
            .json(&json!(stories_yaml))
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await;

        match req {
            Ok(success) => info!("Historias criadas com sucesso. {:?}", success),
            Err(error) => error!("Erro ao criar historias {:?}", error),
        }
    }
}
