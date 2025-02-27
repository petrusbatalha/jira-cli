use crate::commons::req_builder::build_get_req;
use crate::commons::structs::{AuthOptions, Issue, JQL, REST_URI};
use crate::epics::epics_projects::{Epic, EpicHandler};
use crate::EpicOps;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};
use url::Url;

// Query para listar epicos no jira.
// https://jira.bradesco.com.br:8443/rest/api/2/search?jql=PROJECT=ESTRT AND issuetype="Epic"&fields=summary

impl EpicHandler {
    pub async fn list(&self, options: &EpicOps, auth_options: &AuthOptions) {
        let uri = format!("{}{}", &auth_options.host, &REST_URI);
        let project = options.project_key.clone();
        let jql_query = format!(
            "{}{}{}{}{}",
            &uri, &JQL, "PROJECT=", project, " AND issuetype=Epic&fields=summary,description"
        );
        let url = Url::parse(&jql_query).unwrap();

        let epics = build_get_req(url, auth_options)
            .send()
            .await
            .unwrap()
            .json::<Epic>()
            .await
            .unwrap();

        let mut table = Table::new();
        table.max_column_width = 80;
        table.style = TableStyle::blank();
        table.add_row(build_table_header_row());

        for issue in epics.issues.unwrap() {
            table.add_row(build_table_body(issue));
        }

        info!("{}", table.render());
    }
}

fn build_table_body(issue: Issue) -> Row<'static> {
    let fields = issue.fields.unwrap();
    Row::new(vec![
        TableCell::new_with_alignment(fields.summary.unwrap(), 1, Alignment::Left),
        TableCell::new_with_alignment(issue.key, 2, Alignment::Left),
        TableCell::new_with_alignment(issue.id, 1, Alignment::Left),
        TableCell::new_with_alignment(issue.issue_link, 1, Alignment::Left),
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
