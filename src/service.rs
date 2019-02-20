use crate::domain::{ContentDTO, Item, ItemList, Link};

pub fn convert_content_to_item(content: &ContentDTO) -> Item {
    Item {
        text: content.text.to_owned(),
        link: Link { url: content.text.to_owned() },
        created_at: "".to_string(),
    }
}

pub fn add_item_to_list(item: Item, list: &mut ItemList) {
    list.insert(0, item);
}

#[cfg(test)]
mod tests {
    use crate::domain::{ContentDTO, Item, ItemList, Link};
    use super::{convert_content_to_item, add_item_to_list};

    #[test]
    fn should_create_item_from_content_text() {
        // given
        let content = ContentDTO { text: String::from("https://example.com") };

        // when
        let item = convert_content_to_item(&content);

        // then
        assert_eq!(item.text, "https://example.com");
        assert_eq!(item.link.url, "https://example.com");
    }

    #[test]
    fn should_add_item_to_the_beginning_of_the_list() {
        // given
        let mut list: ItemList = vec![
            Item {
                created_at: String::from("2018-02-07T20:43:44"),
                link: Link { url: String::from("https://example.com") },
                text: String::from("Some link"),
            },
            Item {
                created_at: String::from("2018-01-29T21:10:00"),
                link: Link { url: String::from("https://example.com/second") },
                text: String::from("Second link"),
            },
        ];

        let item = Item {
            created_at: String::from("2018-02-10T10:00:00"),
            link: Link { url: String::from("https://example.com/new-link") },
            text: String::from("Next link"),
        };

        // when
        add_item_to_list(item, &mut list);

        // then
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].text, String::from("Next link"));
        assert_eq!(list[1].text, String::from("Some link"));
        assert_eq!(list[2].text, String::from("Second link"));
    }
}
