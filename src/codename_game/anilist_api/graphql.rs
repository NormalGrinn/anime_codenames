pub const LIST_QUERY: &str = "
query ($userName: String)
{
  MediaListCollection (userName: $userName, type:ANIME, status:COMPLETED) {
    lists {
      entries {
        media {
        title {
          romaji
          english
        }
        }
      }
    }
  }
}
";