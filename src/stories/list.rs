use crate::commons::custom_fields::CustomFieldsHandler;
use crate::commons::req_builder::build_get_req;
use crate::commons::structs::{AuthOptions, Issue, JQL, REST_URI};
use crate::stories::stories_structs::{StoriesHandler, StoryResponse};
use crate::StoryListOps;
use anyhow::{bail, Error};
use reqwest::Url;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

impl StoriesHandler {
    pub async fn list(&self, options: &StoryListOps, auth_options: &AuthOptions) {
        let uri = format!("{}{}", &auth_options.host, &REST_URI);

        let custom_fields = match CustomFieldsHandler
            .get_or_cache(auth_options, &options.project)
            .await
        {
            Some((_, rcf)) => {
                Some(rcf)
            },
            _ => None,
        }
        .unwrap();

        debug!("Custom  fields {:?}", &custom_fields);

        let epic_link = custom_fields.get("Epic Link").unwrap();

        let epic_uri = format!(
            "{}{}{}{}{}",
            &uri,
            &JQL,
            &epic_link,
            "=",
            &options.epic.clone()
        );

        let url = Url::parse(&epic_uri).unwrap();

        debug!("Epic Request {}", url);

        let stories = build_get_req(url, auth_options)
            .send()
            .await
            .unwrap()
            .json::<StoryResponse>()
            .await
            .unwrap();

        let mut table = Table::new();
        table.max_column_width = 80;
        table.style = TableStyle::blank();

        table.add_row(build_table_header_row());

        for issue in stories.issues.unwrap() {
            table.add_row(build_table_body(issue));
        }

        print!("{}", table.render());
    }
}

fn build_table_body(stories: Issue) -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment(stories.fields.unwrap().summary.unwrap(), 1, Alignment::Left),
        TableCell::new_with_alignment(stories.key, 1, Alignment::Left),
        TableCell::new_with_alignment(stories.id, 2, Alignment::Left),
        TableCell::new_with_alignment(stories.issue_link, 1, Alignment::Left),
    ])
}

fn build_table_header_row() -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment("Name", 1, Alignment::Left),
        TableCell::new_with_alignment("Key", 2, Alignment::Left),
        TableCell::new_with_alignment("ID", 1, Alignment::Left),
        TableCell::new_with_alignment("Link", 1, Alignment::Left),
    ])
}
