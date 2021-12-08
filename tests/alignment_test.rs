use crate::util::create_vector;
use tabled::{Alignment, Column, Full, Head, Indent, Modify, Row, Style, Table};

mod util;

#[test]
fn full_alignment() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()))
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 | 0-0      | 0-1      | 0-2      \n",
        " 1 | 1-0      | 1-1      | 1-2      \n",
        " 2 | 2-0      | 2-1      | 2-2      \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn head_and_data_alignment() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::ASCII)
        .with(Modify::new(Head).with(Alignment::left()))
        .with(Modify::new(Row(1..)).with(Alignment::right()))
        .to_string();

    let expected = concat!(
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 0 |      0-0 |      0-1 |      0-2 |\n",
        "+---+----------+----------+----------+\n",
        "| 1 |      1-0 |      1-1 |      1-2 |\n",
        "+---+----------+----------+----------+\n",
        "| 2 |      2-0 |      2-1 |      2-2 |\n",
        "+---+----------+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn full_alignment_multiline() {
    let mut data = create_vector::<3, 3>();
    data[2][2] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 | 0-0      | 0-1      | 0-2      \n",
        " 1 | 1-0      | 1-1      | 1-2      \n",
        " 2 | 2-0      | https:// | 2-2      \n",
        "   |          | www      |          \n",
        "   |          | .        |          \n",
        "   |          | redhat   |          \n",
        "   |          | .com     |          \n",
        "   |          | /en      |          \n",
    );

    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Alignment::left()))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn vertical_alignment_test() {
    let mut data = create_vector::<3, 3>();
    data[1][2] = String::from("E\nnde\navou\nros");
    data[2][2] = String::from("Red\nHat");
    data[2][3] = String::from("https://\nwww\n.\nredhat\n.com\n/en");

    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Column(1..)).with(Alignment::bottom()))
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |          |    E     |          \n",
        "   |          |   nde    |          \n",
        "   |          |   avou   |          \n",
        "   |   1-0    |   ros    |   1-2    \n",
        " 2 |          |          | https:// \n",
        "   |          |          |   www    \n",
        "   |          |          |    .     \n",
        "   |          |          |  redhat  \n",
        "   |          |   Red    |   .com   \n",
        "   |   2-0    |   Hat    |   /en    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn alignment_doesnt_change_indent() {
    let data = create_vector::<3, 3>();

    let expected = concat!(
        "   N|   column 0|   column 1|   column 2\n",
        "----+-----------+-----------+-----------\n",
        "   0|   0-0     |   0-1     |   0-2     \n",
        "   1|   1-0     |   1-1     |   1-2     \n",
        "   2|   2-0     |   2-1     |   2-2     \n",
    );

    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(Modify::new(Full).with(Indent::new(3, 0, 0, 0)))
        .with(Modify::new(Full).with(Alignment::left()))
        .to_string();

    assert_eq!(table, expected);
}
