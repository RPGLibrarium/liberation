type Id = u64;
type RpgSystemId = Id;
type TitleId = Id;
type BookId = Id;
type EntityId = Id;
type MemberId = EntityId;
type GuildId = EntityId;
type RentalId = Id;

type Year = u16;
type Date = String;

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
    pub coverimage: String,
}

pub struct Book {
    pub id: BookId,
    pub title: TitleId,
    pub owner_type: String,
    pub owner: EntityId,
    pub quality: String,
}

pub struct Member {
    pub id: MemberId,
    pub external_id: Id,
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
    pub rentee_type: String,
    pub rentee: EntityId,
}
