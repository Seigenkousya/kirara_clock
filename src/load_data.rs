use std::fs::{self};
use yew::{html, Html};
use toml::value::Datetime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Clock {
    date: Datetime,
    remain: Datetime,
    event: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timeline {
    clock: Vec<Clock>,
}

impl Clock {
    pub fn html_row(&self) -> Html {
        html! {
            <tr>
                <td> { self.date.clone() } </td>
                <td> { self.remain.clone() } </td>
                <td> { self.event.clone() } </td>
                <td> { self.description.clone() } </td>
            </tr>
        }
    }
}

impl Timeline {
    fn create_table_header(&self) -> Html {
        html! {
            <tr>
                <th> { "date" } </th>
                <th> { "remain" } </th>
                <th> { "event" } </th>
                <th> { "description" } </th>
            </tr>
        }
    }

    pub fn create_html_table(&self) -> Html {
        let clocks = 
            self.clock
                .iter()
                .map(|clk: &Clock| clk.html_row());
        let table_header = self.create_table_header();

        html! {
            <table>
                { table_header }
                { for clocks }
            </table>
        }
    }
}

pub fn read_toml() -> Result<Timeline, Box<dyn std::error::Error>> {
    //let raw_data: String = fs::read_to_string("timeline.toml")?;
    let raw_data: String = "\
[[clock]]
date = 2019-03-19
remain = 00:00:45
event = 'abcdefg'
description = \"\"\"\\
    god is dead.\
    \"\"\"

[[clock]]
date = 2021-08-19
remain = 00:00:45
event = 'hijklmn'
description = \"\"\"\\
    It's a resurrection day.\\
    \"\"\"
".to_string();

    let parsed_toml: Result<Timeline, toml::de::Error> = toml::from_str(&raw_data);
    let timeline = match parsed_toml {
        Ok(tl) => tl,
        Err(msg) => panic!("Filed to parse TOML: {}", msg),
    };
    Ok(timeline)
}
