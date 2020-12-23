use crate::commons::req_builder::build_req;
use crate::commons::structs::{AuthOptions, REST_URI};
use crate::projects::command_args::ProjectOps;
use crate::projects::projects_structs::ProjectHandler;
use crate::projects::projects_structs::{Project, PROJECT_URI};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};
use url::Url;

impl ProjectHandler {
    pub async fn list(&self, _options: &ProjectOps, auth_options: &AuthOptions) {
        let url = Url::parse(&format!(
            "{}{}{}",
            &auth_options.host, &REST_URI, &PROJECT_URI
        ))
        .unwrap();

        debug!("Listing projects... will call uri: {}", url.clone());

        let projects = build_req(url, auth_options)
            .send()
            .await
            .unwrap()
            .json::<Vec<Project>>()
            .await
            .unwrap();

        let mut table = Table::new();
        table.max_column_width = 40;
        table.style = TableStyle::blank();

        table.add_row(build_table_header_row());

        for project in projects.clone() {
            table.add_row(build_table_body(project));
        }

        println!("{}", table.render());
    }
}

fn build_table_body(project: Project) -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment(project.key.unwrap(), 1, Alignment::Left),
        TableCell::new_with_alignment(project.name.unwrap(), 2, Alignment::Left),
        TableCell::new_with_alignment(project.id.unwrap(), 1, Alignment::Left),
    ])
}

fn build_table_header_row() -> Row<'static> {
    Row::new(vec![
        TableCell::new_with_alignment("Key", 1, Alignment::Left),
        TableCell::new_with_alignment("Name", 2, Alignment::Left),
        TableCell::new_with_alignment("ID", 1, Alignment::Left),
    ])
}
