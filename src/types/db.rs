use chrono::{ self, Utc };
use mongodb;
use uuid;

pub enum TicketStatus {
  Open,
  InProgress,
  Freezed,
  Closed,
  Archived,
}

pub struct Ticket {
  pub user_id: i64, // The discord ID of the owner of this ticket.
  pub server_id: i64, // The server ID of this ticket.
  pub channel_id: i64, // The channel ID assigned to this ticket.

  pub ticket_id: uuid::Uuid, // The unique identifier for this ticket.

  pub created_at: chrono::DateTime<chrono::Utc>, // Date and time of creation.
  pub updated_at: chrono::DateTime<chrono::Utc>, // Date and time of last update.

  pub status: TicketStatus, // The status of the ticket.
}

// Convertion from ticket to BSON and back.
impl From<Ticket> for mongodb::bson::Document {
  fn from(value: Ticket) -> Self {
    let mut doc = mongodb::bson::Document::new();
    doc.insert("user_id", value.user_id);
    doc.insert("server_id", value.server_id);
    doc.insert("channel_id", value.channel_id);

    doc.insert("ticket_id", value.ticket_id);

    doc.insert("created_at", value.created_at);
    doc.insert("updated_at", value.updated_at);

    doc.insert("status", value.status as i64);

    doc
  }
}
impl From<mongodb::bson::Document> for Ticket {
  fn from(value: mongodb::bson::Document) -> Self {
    Ticket {
      user_id: value.get("user_id").unwrap().as_i64().unwrap(),
      server_id: value.get("server_id").unwrap().as_i64().unwrap(),
      channel_id: value.get("channel_id").unwrap().as_i64().unwrap(),
      ticket_id: bson_to_uuid(value.get("ticket_id").unwrap().to_owned()),
      created_at: chrono::DateTime::<Utc>::from(
        value.get("created_at").unwrap().as_datetime().unwrap().to_owned()
      ),
      updated_at: chrono::DateTime::<Utc>::from(
        value.get("updated_at").unwrap().as_datetime().unwrap().to_owned()
      ),
      status: TicketStatus::from(value.get("status").unwrap().as_i64().unwrap()),
    }
  }
}

// Ticket status is stored as an integer.
impl From<TicketStatus> for i64 {
  fn from(value: TicketStatus) -> Self {
    match value {
      TicketStatus::Open => 1,
      TicketStatus::InProgress => 2,
      TicketStatus::Freezed => 3,
      TicketStatus::Closed => 4,
      TicketStatus::Archived => 5,
      _ => unreachable!("Invalid ticket status"),
    }
  }
}
impl From<i64> for TicketStatus {
  fn from(value: i64) -> Self {
    match value {
      1 => TicketStatus::Open,
      2 => TicketStatus::InProgress,
      3 => TicketStatus::Freezed,
      4 => TicketStatus::Closed,
      5 => TicketStatus::Archived,
      _ => unreachable!("Invalid ticket status"),
    }
  }
}
fn bson_to_uuid(doc: mongodb::bson::Bson) -> uuid::Uuid {
  uuid::Uuid::parse_str(&doc.to_string()).unwrap()
}

pub enum ChannelType {
  Generic,
  News,
  Bots,
  Neon,
  Spam,
}

impl From<ChannelType> for i64 {
  fn from(value: ChannelType) -> Self {
    match value {
      ChannelType::Generic => 1,
      ChannelType::News => 2,
      ChannelType::Bots => 3,
      ChannelType::Neon => 4,
      ChannelType::Spam => 5,
      _ => panic!("Invalid channel type"),
    }
  }
}

impl From<i64> for ChannelType {
  fn from(value: i64) -> Self {
    match value {
      1 => ChannelType::Generic,
      2 => ChannelType::News,
      3 => ChannelType::Bots,
      4 => ChannelType::Neon,
      5 => ChannelType::Spam,
      _ => panic!("Invalid channel type"),
    }
  }
}

pub struct ChannelPar {
  channel_id: i64,
  guild_id: i64,
  channel_type: ChannelType,
}

impl From<ChannelPar> for mongodb::bson::Bson {
  fn from(value: ChannelPar) -> Self {
    let mut doc = mongodb::bson::Document::new();
    doc.insert("channel_id", value.channel_id);
    doc.insert("guild_id", value.guild_id);

    let cht: i64 = value.channel_type.into();
    doc.insert("channel_type", cht);

    mongodb::bson::Bson::Document(doc)
  }
}

impl From<mongodb::bson::Document> for ChannelPar {
  fn from(value: mongodb::bson::Document) -> Self {
    ChannelPar {
      channel_id: value.get("channel_id").unwrap().as_i64().unwrap().to_owned(),
      guild_id: value.get("guild_id").unwrap().as_i64().unwrap().to_owned(),
      channel_type: ChannelType::from(value.get("channel_type").unwrap().as_i64().unwrap()),
    }
  }
}

fn parse_chn_pr(docs: Vec<mongodb::bson::Bson>) -> Vec<ChannelPar> {
  docs.iter()
    .map(|doc| {
      match doc {
        mongodb::bson::Bson::Document(doc) => ChannelPar::from(doc.to_owned()),
        _ => panic!("Invalid bson type"),
      }
    })
    .collect()
}

pub struct server {
  server_id: i64,

  welcome_channel_id: i64,
  channel_vec: Vec<ChannelPar>,
}

// Convertion from server to BSON and back.
impl From<server> for mongodb::bson::Document {
  fn from(value: server) -> Self {
    let mut doc = mongodb::bson::Document::new();
    doc.insert("server_id", value.server_id);
    doc.insert("welcome_channel_id", value.welcome_channel_id);
    doc.insert("channel_vec", value.channel_vec);

    doc
  }
}
impl From<mongodb::bson::Document> for server {
  fn from(value: mongodb::bson::Document) -> Self {
    server {
      server_id: value.get("server_id").unwrap().as_i64().unwrap().to_owned(),
      welcome_channel_id: value
        .get("welcome_channel_id")
        .unwrap()
        .as_i64()
        .unwrap()
        .to_owned(),
      channel_vec: parse_chn_pr(
        value.get("channel_vec").unwrap().as_array().unwrap().to_owned()
      ),
    }
  }
}

pub struct user {
  user_id: i64,
  prefered_language: (String, f64),
  spoken_languages: Vec<(String, f64)>,

  email: String,
}

impl From<user> for mongodb::bson::Document {
  fn from(value: user) -> Self {
    let mut doc = mongodb::bson::Document::new();
    doc.insert("user_id", value.user_id);
    // Convert languages
    let mut prefered_language = mongodb::bson::Document::new();
    prefered_language.insert("name", value.prefered_language.0);
    prefered_language.insert("value", value.prefered_language.1);

    doc.insert("prefered_language", prefered_language);

    // Convert spoken language to an array of document that have the same syntax as prefered_language
    // We just have to make a vecor with the document in it.
    let mut spoken_languages: Vec<mongodb::bson::Document> = Vec::new();
    for (name, value) in value.spoken_languages {
      let mut doc = mongodb::bson::Document::new();
      doc.insert("name", name);
      doc.insert("value", value);
      spoken_languages.push(doc);
    }
    doc.insert("spoken_languages", spoken_languages);

    doc.insert("email", value.email);

    doc
  }
}

impl From<mongodb::bson::Document> for user {
  fn from(value: mongodb::bson::Document) -> Self {
    let prefered_language = value.get("prefered_language").unwrap().as_document().unwrap();
    let mut spoken_languages: Vec<(String, f64)> = Vec::new();
    // We do the same process but in reverse
    for doc in value.get("spoken_languages").unwrap().as_array().unwrap() {
      let docp = doc.as_document().unwrap();
      let lang_name = docp.get("name").unwrap().as_str().unwrap();
      let lang_score = docp.get("value").unwrap().as_f64().unwrap();
      spoken_languages.push((lang_name.to_owned(), lang_score));
    }

    user {
      user_id: value.get("user_id").unwrap().as_i64().unwrap().to_owned(),
      prefered_language: (
        prefered_language.get("name").unwrap().as_str().unwrap().to_owned(),
        prefered_language.get("name").unwrap().as_f64().unwrap().to_owned(),
      ),
      spoken_languages: spoken_languages,
      email: value.get("email").unwrap().as_str().unwrap().to_owned(),
    }
  }
}