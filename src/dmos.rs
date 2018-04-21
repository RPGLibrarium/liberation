pub type Id = u64;
pub type RpgSystemId = Id;
pub type TitleId = Id;
pub type BookId = Id;
pub type EntityId = Id;
pub type MemberId = EntityId;
pub type GuildId = EntityId;
pub type RentalId = Id;

pub type Year = u16;
pub type Date = String;

pub enum EntityType {
    Member,
    Guild,
}

impl EntityType {
    pub fn from_str(s: &str) -> Result<EntityType, ()>{
        match s {
            "member" => Ok(EntityType::Member),
            "guild" => Ok(EntityType::Guild),
            _ => Err(()),
        }
    }

    pub fn to_string(self) -> String{
        match self{
            EntityType::Member => String::from("member"),
            EntityType::Guild => String::from("guild"),
        }
    }
}

#[derive(Debug)]
pub struct RpgSystem {
    pub id: RpgSystemId,
    pub name: String,
}

pub struct Title {
    pub id: TitleId,
    pub name: String,
    pub system: RpgSystemId,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

pub struct Book {
    pub id: BookId,
    pub title: TitleId,
    pub owner_type: EntityType,
    pub owner: EntityId,
    pub quality: String,
}

pub struct Member {
    pub id: MemberId,
    pub external_id: String,
}

pub struct Guild {
    pub id: GuildId,
    pub name: String,
    pub address: String,
    pub contact: MemberId,
}

pub struct Rental {
    pub id: RentalId,
    pub from: Date,
    pub to: Date,
    pub book: BookId,
    pub rentee_type: EntityType,
    pub rentee: EntityId,
}

impl Book {
    pub fn new(id: BookId, title: TitleId, owner: EntityId, owner_type: EntityType, quality: String) -> Book {
        return Book {
            id: id,
            title: title,
            owner_type: owner_type,
            owner: owner,
            quality: quality,
        }
    }

    pub fn from_db(id: BookId, title: TitleId, owner_member: Option<MemberId>, owner_guild: Option<GuildId>, owner_type: String, quality: String) -> Result<Book, String> {
        let owner_type = match EntityType::from_str(owner_type.as_str()) {
            Ok(x) => x,
            Err(_) => return Err(String::from("Bad owner_type")),
        };

        let owner: EntityId = match match owner_type {
            EntityType::Member => owner_member,
            EntityType::Guild => owner_guild,
        } {
            Some(x) => x,
            None => return Err(String::from("Field 'owner_member' or 'owner_guild' is not set according to 'owner_type'.")),
        };

        Ok(Book::new(id, title,  owner, owner_type, quality))
    }
}

impl Rental {
    pub fn new(id: RentalId, from: Date, to: Date, book: BookId, rentee: EntityId, rentee_type: EntityType) -> Rental {
        return Rental {
            id: id,
            from: from,
            to: to,
            book: book,
            rentee: rentee,
            rentee_type: rentee_type,
        }
    }

    pub fn from_db(id: RentalId, from: Date, to: Date, book: BookId, rentee_member: Option<MemberId>, rentee_guild: Option<GuildId>, rentee_type: String) -> Result<Rental, String> {
        let rentee_type = match EntityType::from_str(rentee_type.as_str()) {
            Ok(x) => x,
            Err(_) => return Err(String::from("Bad rentee_type")),
        };

        let rentee: EntityId = match match rentee_type {
            EntityType::Member => rentee_member,
            EntityType::Guild => rentee_guild,
        } {
            Some(x) => x,
            None => return Err(String::from("Field 'rentee_member' or 'rentee_guild' is not set according to 'rentee_type'.")),
        };

        Ok(Rental::new(id, from, to, book, rentee, rentee_type))
    }
}
