use crate::commons::custom_fields::CustomFieldsHandler;
use crate::commons::req_builder::build_req;
use crate::commons::structs::{AuthOptions, Issue, JQL, REST_URI};
use crate::stories::stories_structs::{StoryMeta, StoriesHandler};
use crate::StoryListOps;
use reqwest::Url;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

impl StoriesHandler {
    pub async fn list(&self, options: &StoryListOps, auth_options: &AuthOptions) {
        let uri = format!("{}{}", &auth_options.host, &REST_URI);

        let custom_fields = CustomFieldsHandler
            .get_or_cache(auth_options, &options.project)
            .await
            .unwrap();

        let epic_link = custom_fields.get("Epic Link").unwrap();
        let epic_field = format!("cf[{}]", epic_link.replace("customfield_", ""));

        let epic_uri = format!(
            "{}{}{}{}{}",
            &uri,
            &JQL,
            &epic_field,
            "=",
            &options.epic.clone()
        );

        let url = Url::parse(&epic_uri).unwrap();

        debug!("Epic Request {}", url);

        let stories = build_req(url, auth_options)
            .send()
            .await
            .unwrap()
            .json::<StoryMeta>()
            .await
            .unwrap();

        let mut table = Table::new();
        table.max_column_width = 80;
        table.style = TableStyle::blank();

        table.add_row(build_table_header_row());

        for issue in stories.issues.unwrap() {
            table.add_row(build_table_body(issue));
        }

        println!("{}", table.render());
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
