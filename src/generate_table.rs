use prettytable::{Attr, color,Table, Row, Cell};

// Adding data to the able inplace
pub fn adding_row(table:& mut Table,usn:&str, address: &str, server: &str){

    table.add_row(Row::new(vec![
        Cell::new(&usn),
        Cell::new(&address)cha,
        Cell::new(&server),
    ]));
}

// Creating and style table
pub fn creating_table()->Table{
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Usn")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Italic(true))
            .with_hspan(1),

        Cell::new("Address")
            .with_style(Attr::ForegroundColor(color::RED))
            .with_style(Attr::Italic(true))
            .with_hspan(1),
        Cell::new("Server")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Italic(true))
            .with_hspan(1)

    ]));
    table
}