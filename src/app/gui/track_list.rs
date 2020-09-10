#[derive(Clone, Debug)]
pub enum TrackListMessage {
    AddTrack(String),
}
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct RowFormat {
    pub title_width: iced::Length,
    pub artist_width: iced::Length,
}
#[derive(Clone, Debug)]
pub struct TrackRow {
    pub title: String,
    pub artist: String,
}
impl TrackRow {
    pub fn title_formatted(&self, width: iced::Length) -> iced::Text {
        iced::Text::new(self.title.as_str()).width(width)
    }
    pub fn artist_formatted(&self, width: iced::Length) -> iced::Text {
        iced::Text::new(self.artist.as_str()).width(width)
    }
    pub fn row(&self, row_format: &RowFormat) -> iced::Row<super::application::Message> {
        iced::Row::new()
            .push(self.title_formatted(row_format.title_width))
            .push(self.artist_formatted(row_format.artist_width))
    }
}
#[derive(Clone, Debug)]
pub struct TrackList {
    pub format: RowFormat,
    pub rows: Vec<TrackRow>,
}
impl TrackList {
    pub fn as_element(&self) -> iced::Element<super::application::Message> {
        iced::Column::with_children(
            self.rows
                .iter()
                .map(|r| r.row(&self.format))
                .map(iced::Element::from)
                .collect(),
        )
        .into()
    }
}
