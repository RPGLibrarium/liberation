use database as db;
use serde_formats;

type ItemCount = u32;

// We use some keywords in the definition of DTOS:
// Partial: There is no Primary Id specified. And foreign keys are not resolved. Usually used in Post and Put
// Unresolved: Only Ids without content.
// ############### Response/Outer DTOS #########################

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetRpgSystems {
    pub rpgsystems: Vec<db::RpgSystem>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetRpgSystem {
    pub rpgsystem: RpgSystemWithTitles,
}

impl GetRpgSystem {
    pub fn new(rpg_system: db::RpgSystem, titles: Vec<db::Title>) -> GetRpgSystem {
        return GetRpgSystem {
            rpgsystem: RpgSystemWithTitles {
                id: rpg_system.id.expect("RpgSystem must contain an Id"),
                name: rpg_system.name,
                titles,
            },
        };
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PutPostRpgSystem {
    pub rpgsystem: db::RpgSystem,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetTitles {
    pub titles: Vec<TitleWithSystem>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetTitle {
    pub title: TitleWithSystemWithBooks,
}

impl GetTitle {
    pub fn new(
        title: db::Title,
        system: db::RpgSystem,
        stock: u32,
        available: u32,
        books: Vec<BookWithOwnerWithRental>,
    ) -> GetTitle {
        GetTitle {
            title: TitleWithSystemWithBooks {
                id: title.id.unwrap(),
                name: title.name,
                system: system,
                language: title.language,
                publisher: title.publisher,
                year: title.year,
                coverimage: title.coverimage,
                stock,
                available,
                books,
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PutPostTitle {
    pub title: db::Title,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetBooks {
    pub books: Vec<BookWithTitleWithOwnerWithRental>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetBook {
    pub book: BookWithTitleWithOwnerWithRental,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PutPostBook {
    pub book: PartialBook,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetMembers {
    pub members: Vec<MemberWithRoles>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetMember {
    pub member: MemberWithRoles,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetMemberInventory {
    pub member: Entity,
    pub inventory: Inventory,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PutMemberInvetory {
    pub book: PartialBook,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetGuilds {
    pub guilds: Vec<GuildWithContact>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetGuild {
    pub guild: GuildWithContact,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub struct GetGuildInventory {
    pub guild: Entity,
    pub inventory: Inventory,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PutPostGuild {
    pub guild: PartialGuild,
}

// ############### Inner DTOS ############################
#[derive(Serialize)]
pub struct RpgSystemWithTitles {
    pub id: db::RpgSystemId,
    pub name: String,
    pub titles: Vec<db::Title>,
}

#[derive(Deserialize)]
pub struct PartialRpgSystem {
    pub name: String,
}

#[derive(Serialize, Clone)]
pub struct TitleWithSystem {
    pub id: db::TitleId,
    pub name: String,
    pub system: db::RpgSystem,
    pub language: String,
    pub publisher: String,
    pub year: db::Year,
    pub coverimage: Option<String>,
    pub stock: ItemCount,
    pub available: ItemCount,
}

impl TitleWithSystem {
    pub fn new(
        title: db::Title,
        system: db::RpgSystem,
        stock: u32,
        available: u32,
    ) -> TitleWithSystem {
        TitleWithSystem {
            id: title.id.expect("Expected a title id"),
            name: title.name,
            system,
            language: title.language,
            publisher: title.publisher,
            year: title.year,
            coverimage: title.coverimage,
            stock,
            available,
        }
    }
}

#[derive(Serialize)]
pub struct TitleWithSystemWithBooks {
    pub id: db::TitleId,
    pub name: String,
    pub system: db::RpgSystem,
    pub language: String,
    pub publisher: String,
    pub year: db::Year,
    pub coverimage: Option<String>,
    pub stock: ItemCount,
    pub available: ItemCount,
    pub books: Vec<BookWithOwnerWithRental>,
}

#[derive(Serialize)]
pub struct BookWithOwnerWithRental {
    pub id: db::BookId,
    pub owner: Entity,
    pub quality: String,
    pub available: bool,
    pub rental: RentalWithRentee,
}

#[derive(Serialize)]
pub struct Entity {
    #[serde(rename = "type")]
    pub entity_type: db::EntityType,
    pub id: db::Id,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UnresolvedEntity {
    #[serde(rename = "type")]
    pub entity_type: db::EntityType,
    pub id: db::Id,
}

#[derive(Serialize)]
pub struct RentalWithRentee {
    #[serde(with = "serde_formats::naive_date")]
    pub from: db::Date,
    #[serde(with = "serde_formats::naive_date")]
    pub to: db::Date,
    pub rentee: Entity,
}

#[derive(Deserialize)]
pub struct PartialTitle {
    pub name: String,
    pub system: db::TitleId,
    pub language: String,
    pub publisher: String,
    pub year: db::Year,
    pub coverimage: Option<String>,
}

#[derive(Serialize)]
pub struct BookWithTitleWithOwnerWithRental {
    pub id: db::BookId,
    pub title: TitleWithSystem,
    pub owner: Entity,
    pub quality: String,
    pub available: bool,
    pub rental: Option<RentalWithRentee>,
}

#[derive(Deserialize)]
pub struct PartialBook {
    pub title: db::TitleId,
    pub owner: UnresolvedEntity,
    pub quality: String,
}

#[derive(Serialize)]
pub struct Inventory {
    pub ownedbookds: Vec<BookWithTitleWithOwnerWithRental>,
    pub rentedbooks: Vec<BookWithOwnerWithRental>,
}

#[derive(Serialize)]
pub struct MemberWithRoles {
    pub id: db::MemberId,
    pub name: String,
    pub email: String,
    pub roles: Vec<db::Role>,
}

#[derive(Serialize)]
pub struct Member {
    pub id: db::MemberId,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct GuildWithContact {
    pub id: db::GuildId,
    pub name: String,
    pub address: String,
    pub contact: Member,
}

#[derive(Deserialize)]
pub struct PartialGuild {
    pub name: String,
    pub address: String,
    pub contact: db::MemberId,
}
