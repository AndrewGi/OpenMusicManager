pub struct JustTableState {
    pub row_height: iced::Length,
    pub column_widths: Vec<iced::Length>,
}
impl JustTableState {
    pub fn new(row_height: iced::Length, column_widths: Vec<iced::Length>) -> JustTableState {
        JustTableState {
            column_widths,
            row_height,
        }
    }
    pub fn format_row<'a, Message: 'a>(
        &self,
        values: impl Iterator<Item = iced::Element<'a, Message>>,
    ) -> iced::Row<'a, Message> {
        let cols_count = self.column_widths.len();
        // Make sure the amount of row values is equal to the number of columns
        let mut index = 0;
        let out = iced::Row::with_children(
            values
                .zip(self.column_widths.iter().copied())
                .map(|(e, width)| {
                    index += 1;
                    iced::container::Container::new(e)
                        .width(width)
                        .height(self.row_height)
                })
                .map(From::from)
                .collect(),
        );
        assert_eq!(index, cols_count, "wrong amount of row values");
        out
    }
}
pub struct TableState {
    pub table: JustTableState,
    pub scroll: iced::scrollable::State,
}
impl TableState {
    pub fn new(row_height: iced::Length, col_widths: Vec<iced::Length>) -> TableState {
        TableState {
            table: JustTableState::new(row_height, col_widths),
            scroll: iced::scrollable::State::new(),
        }
    }
}
pub struct Table<'a, Message> {
    pub col: iced::widget::Column<'a, Message>,
    pub state: &'a mut TableState,
}

impl<'a, Message> Table<'a, Message> {
    pub fn with_children(
        state: &'a mut TableState,
        children: Vec<iced::Element<'a, Message>>,
    ) -> Self {
        Table {
            col: iced::widget::Column::with_children(children),
            state,
        }
    }
    pub fn new(state: &'a mut TableState) -> Table<'a, Message> {
        Table {
            col: iced::widget::Column::new().padding(20),
            state,
        }
    }
    pub fn push_inplace<'b: 'a>(&mut self, values: impl Iterator<Item = iced::Element<'b, Message>>)
    where
        Message: 'b,
    {
        let (col, state) = (&mut self.col, &mut self.state);
        take_mut::take(col, |col| col.push(state.table.format_row(values)))
    }
    pub fn push<'b: 'a>(self, values: impl Iterator<Item = iced::Element<'b, Message>>) -> Self
    where
        Message: 'b,
    {
        Table {
            col: self.col.push(self.state.table.format_row(values)),
            state: self.state,
        }
    }
}
impl<'a, Message: 'a> From<Table<'a, Message>> for iced::Element<'a, Message> {
    fn from(t: Table<'a, Message>) -> Self {
        t.col.into()
    }
}
